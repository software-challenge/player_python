"""
This module handels the communication with the api and the students logic.
"""
import logging
import sys
import time
from typing import List, Union

from socha.api.networking._xflux import _XFluxClient
from socha.api.plugin import penguins
from socha.api.plugin.penguins import Field, GameState, Move, CartesianCoordinate
from socha.api.protocol.protocol import State, Board, Data, \
    Error, From, Join, Joined, JoinPrepared, JoinRoom, To, Team, Room, Result, MoveRequest, ObservableRoomMessage, Left
from socha.api.protocol.protocol_packet import ProtocolPacket


def _convertBoard(protocolBoard: Board) -> penguins.Board:
    """
    Converts a protocol Board to a usable game board for using in the logic.
    :rtype: object
    """
    boardList: List[List[Field]] = []
    for y, row in enumerate(protocolBoard.list_value):
        rowList: List[Field] = []
        for x, fieldsValue in enumerate(row.field_value):
            fieldCoordinate = CartesianCoordinate(x, y).to_hex()
            rowList.append(Field(coordinate=fieldCoordinate, field=fieldsValue))
        boardList.append(rowList)
    return penguins.Board(boardList)


class IClientHandler:
    history: List[Union[GameState, Error, Result]] = []

    def calculate_move(self) -> Move:
        """
        Calculates a move that the logic wants the server to perform in the game room.
        """

    def on_update(self, state: GameState):
        """
        If the server _send a update on the current state of the game this method is called.
        :param state: The current state that server sent.
        """

    def on_game_over(self, roomMessage: Result):
        """
        If the game has ended the server will _send a result message.
        This method will called if this happens.

        :param roomMessage: The Result the server has sent.
        """

    def on_error(self, logMessage: str):
        """
        If error occurs,
        for instance when the logic sent a move that is not rule conform,
        the server will _send an error message and closes the connection.
        If this happens, this method is called.

        :param logMessage: The message, that server sent.
        """

    def on_room_message(self, data):
        """
        If the server sends a message that cannot be handelt by anny other method,
        this will be called.

        :param data: The data the Server sent.
        """

    def on_game_prepared(self, message):
        """
        If the game has been prepared by the server this method will be called.

        :param message: The message that server sends with the response.
        """

    def on_game_joined(self, room_id):
        """
        If the client has successfully joined a game room this method will be called.

        :param room_id: The room id the client has joined.
        """

    def on_game_observed(self, message):
        """
        If the client successfully joined as observer this method will be called.

        :param message: The message that server sends with the response.
        """

    def on_game_left(self):
        """
        If the server left the room, this method will be called.
        If the client is running on survive mode it'll be running until shut downed manually.
        """

    def while_disconnected(self, player_client: '_PlayerClient'):
        """
        The client loop will keep calling this method while there is no active connection to a game server.
        This can be used to do tasks after a game is finished and the server left.
        Please be aware, that the client has to be shut down manually if it is in survive mode.
        The return statement is used to tell the client whether to exit or not.

        :type player_client: The player client in which the logic is integrated.
        :return: True if the client should shut down. False if the client should continue to run.
        """


class _PlayerClient(_XFluxClient):
    """
    The PlayerClient handles all incoming and outgoing objects accordingly to their types.
    """

    def __init__(self, host: str, port: int, handler: IClientHandler, survive: bool):
        super().__init__(host, port)
        self._game_handler = handler
        self.survive = survive

    def join_game(self):
        super()._send(Join())

    def join_game_room(self, room_id: str):
        super()._send(JoinRoom(room_id=room_id))

    def join_game_with_reservation(self, reservation: str):
        super()._send(JoinPrepared(reservation_code=reservation))

    def send_message_to_room(self, room_id: str, message):
        super()._send(Room(room_id=room_id, data=message))

    def _on_object(self, message):
        if isinstance(message, Room):
            room_id: str = message.room_id
            data = message.data.class_binding
            if isinstance(data, MoveRequest):
                start_time = time.time()
                response = self._game_handler.calculate_move()
                logging.info(f"Sent {response} after {time.time() - start_time} seconds.")
                if response:
                    from_value = None
                    to = To(x=response.to_value.x, y=response.to_value.y)
                    if response.from_value:
                        from_value = From(x=response.from_value.x, y=response.from_value.y)
                    response = Data(class_value="move", from_value=from_value, to=to)
                    self.send_message_to_room(room_id, response)
            if isinstance(data, ObservableRoomMessage):
                if isinstance(data, State):
                    game_state = GameState(turn=data.turn, start_team=Team(data.start_team),
                                           board=_convertBoard(data.board), last_move=data.last_move,
                                           fishes=penguins.Fishes(data.fishes.int_value[0], data.fishes.int_value[1]))
                    self._game_handler.history.append(game_state)
                    self._game_handler.on_update(game_state)
                elif isinstance(data, Result):
                    self._game_handler.history.append(data)
                    self._game_handler.on_game_over(data)
            if isinstance(data, Error):
                logging.error(data.message)
                self._game_handler.history.append(data)
                self._game_handler.on_error(data.message)
            else:
                self._game_handler.on_room_message(data)
        elif isinstance(message, Joined):
            self._game_handler.on_game_joined(room_id=message.room_id)
        elif isinstance(message, Left):
            self._game_handler.on_game_left()

    def start(self):
        """
        Starts the client loop.
        """
        self._running = True
        self._client_loop()

    def _client_loop(self):
        """
        The client loop is the main loop, where the client waits for messages from the server
        and handles them accordingly.
        """
        while self._running:
            if self._network_interface.connected:
                response = self._receive()
                if isinstance(response, ProtocolPacket):
                    if isinstance(response, Left):
                        if not self.survive:
                            logging.info("The server left.")
                            self.stop()
                        else:
                            logging.info("The server left. Client is in survive mode and keeps running.\n"
                                         "Please shutdown the client manually.")
                            self.close_connection()
                    else:
                        logging.debug(f"Received new object: {response}")
                        self._on_object(response)
                elif self._running:
                    logging.error(f"Received object of unknown class: {response}")
                    raise NotImplementedError("Received object of unknown class.")
            else:
                self._game_handler.while_disconnected(player_client=self)

        logging.info("Done.")
        sys.exit()

    def stop(self):
        """
        Disconnects from the server and stops the client loop.
        """
        logging.info("Shutting down...")
        if self._network_interface.connected:
            self.close_connection()
        self._running = False
