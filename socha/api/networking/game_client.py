"""
This module handels the communication with the api and the students logic.
"""
import logging
import sys
import time
from typing import List, Union

from socha.api.networking.xml_protocol_interface import XMLProtocolInterface
from socha.api.plugin import penguins
from socha.api.plugin.penguins import Field, GameState, Move, CartesianCoordinate, TeamEnum, Penguin, HexCoordinate
from socha.api.protocol.protocol import State, Board, Data, \
    Error, From, Join, Joined, JoinPrepared, JoinRoom, To, Team, Room, Result, MoveRequest, Left
from socha.api.protocol.protocol_packet import ProtocolPacket


def _convert_board(protocol_board: Board) -> penguins.Board:
    """
    Converts a protocol Board to a usable game board for using in the logic.
    :param protocol_board: A Board object in protocol format
    :type protocol_board: Board
    :return: A Board object in the format used by the game logic
    :rtype: penguins.Board
    """
    if not isinstance(protocol_board, Board):
        raise TypeError("The input must be a Board object in protocol format")

    board_list = []
    for y, row in enumerate(protocol_board.list_value):
        board_list.append([])
        for x, fields_value in enumerate(row.field_value):
            coordinate = CartesianCoordinate(x, y).to_hex()
            if type(fields_value) is int:
                board_list[y].append(Field(coordinate, penguin=None, fish=fields_value))
            elif fields_value == "ONE":
                board_list[y].append(Field(coordinate, penguin=Penguin(coordinate, TeamEnum.ONE), fish=0))
            elif fields_value == "TWO":
                board_list[y].append(Field(coordinate, penguin=Penguin(coordinate, TeamEnum.TWO), fish=0))
            else:
                raise ValueError(f"Invalid field value {fields_value} at coordinates {coordinate}")

    return penguins.Board(board_list)


class IClientHandler:
    history: List[List[Union[GameState, Error, Result]]] = []

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

    def while_disconnected(self, player_client: 'GameClient'):
        """
        The client loop will keep calling this method while there is no active connection to a game server.
        This can be used to do tasks after a game is finished and the server left.
        Please be aware, that the client has to be shut down manually if it is in survive mode.
        The return statement is used to tell the client whether to exit or not.

        :type player_client: The player client in which the logic is integrated.
        :return: True if the client should shut down. False if the client should continue to run.
        """


class GameClient(XMLProtocolInterface):
    """
    The PlayerClient handles all incoming and outgoing objects accordingly to their types.
    """

    def __init__(self, host: str, port: int, handler: IClientHandler, reservation: str,
                 room_id: str, auto_reconnect: bool, survive: bool):
        super().__init__(host, port)
        self._game_handler = handler
        self.reservation = reservation
        self.room_id = room_id
        self.auto_reconnect = auto_reconnect
        self.survive = survive

    def join_game(self):
        self._send(Join())

    def join_game_room(self, room_id: str):
        self._send(JoinRoom(room_id=room_id))

    def join_game_with_reservation(self, reservation: str):
        self._send(JoinPrepared(reservation_code=reservation))

    def send_message_to_room(self, room_id: str, message):
        self._send(Room(room_id=room_id, data=message))

    def _on_object(self, message):
        """
        Process various types of messages related to a game.

        Args:
            message: The message object containing information about the game.

        Returns:
            None
        """
        room_id = message.room_id
        if isinstance(message, Joined):
            self._game_handler.on_game_joined(room_id=room_id)
        elif isinstance(message, Left):
            self._game_handler.on_game_left()
        elif isinstance(message.data.class_binding, MoveRequest):
            self._on_move_request(room_id)
        elif isinstance(message.data.class_binding, State):
            self._on_state(message)
        elif isinstance(message.data.class_binding, Result):
            self._game_handler.history[-1].append(message.data.class_binding)
            self._game_handler.on_game_over(message.data.class_binding)
        elif isinstance(message, Room):
            self._game_handler.on_room_message(message.data.class_binding)

    def _on_move_request(self, room_id):
        start_time = time.time()
        move_response = self._game_handler.calculate_move()
        if move_response:
            from_pos = None
            to_pos = To(x=move_response.to_value.x, y=move_response.to_value.y)
            if move_response.from_value:
                from_pos = From(x=move_response.from_value.x, y=move_response.from_value.y)
            response = Data(class_value="move", from_value=from_pos, to=to_pos)
            logging.info(f"Sent {move_response} after {round(time.time() - start_time, ndigits=3)} seconds.")
            self.send_message_to_room(room_id, response)

    def _on_state(self, message):
        last_game_state = None
        for item in self._game_handler.history[-1][::-1]:
            if isinstance(item, GameState):
                last_game_state = item
                break
        if last_game_state:
            from_value = None if not message.data.class_binding.last_move.from_value else HexCoordinate(
                x=message.data.class_binding.last_move.from_value.x,
                y=message.data.class_binding.last_move.from_value.y)
            to_value = HexCoordinate(x=message.data.class_binding.last_move.to.x,
                                     y=message.data.class_binding.last_move.to.y)
            last_move = Move(team_enum=last_game_state.current_team.name,
                             from_value=from_value,
                             to_value=to_value)
            game_state = last_game_state.perform_move(last_move)
        else:
            first_team = Team(TeamEnum.ONE,
                              fish=0 if not last_game_state else last_game_state.first_team.fish,
                              penguins=[] if not last_game_state else last_game_state.first_team.penguins,
                              moves=[] if not last_game_state else last_game_state.first_team.moves)
            second_team = Team(TeamEnum.TWO,
                               0 if not last_game_state else last_game_state.second_team.fish,
                               penguins=[] if not last_game_state else last_game_state.second_team.penguins,
                               moves=[] if not last_game_state else last_game_state.second_team.moves)

            game_state = GameState(
                board=_convert_board(message.data.class_binding.board),
                turn=message.data.class_binding.turn,
                first_team=first_team,
                second_team=second_team,
                last_move=None,
            )
        self._game_handler.history[-1].append(game_state)
        self._game_handler.on_update(game_state)

    def start(self):
        """
        Starts the client loop.
        """
        self.running = True
        self._client_loop()

    def join(self) -> None:
        """
        Tries to join a game with a connected server. It uses the reservation, or room id to join.
        If their are not present it joins just without, as fallback.
        """
        if self.reservation:
            self.join_game_with_reservation(self.reservation)
        elif self.room_id:
            self.join_game_room(self.room_id)
        else:
            self.join_game()

        self.first_time = False
        self._game_handler.history.append([])

    def _handle_left(self):
        self.first_time = True
        self.disconnect()
        if self.survive:
            logging.info("The server left. Client is in survive mode and keeps running.\n"
                         "Please shutdown the client manually.")
            self._game_handler.while_disconnected(player_client=self)
        if self.auto_reconnect:
            logging.info("The server left. Client tries to reconnect to the server.")
            for _ in range(3):
                logging.info(f"Try to establish a connection with the server...")
                try:
                    self.connect()
                    if self.network_interface.connected:
                        logging.info("Reconnected to server.")
                        break
                except Exception as e:
                    logging.exception(e)
                    logging.info("The client couldn't reconnect due to a previous error.")
                    self.stop()
                time.sleep(1)
            self.join()
            return
        logging.info("The server left.")
        self.stop()

    def _client_loop(self):
        """
        The client loop is the main loop, where the client waits for messages from the server
        and handles them accordingly.
        """

        while self.running:
            if self.network_interface.connected:
                response = self._receive()
                if not response:
                    continue
                elif isinstance(response, ProtocolPacket):
                    logging.debug(f"Received new object: {response}")
                    if isinstance(response, Left):
                        self._handle_left()
                    else:
                        self._on_object(response)
                elif self.running:
                    logging.error(f"Received a object of unknown class: {response}")
                    raise NotImplementedError("Received object of unknown class.")
            else:
                self._game_handler.while_disconnected(player_client=self)

        logging.info("Done.")
        sys.exit(0)

    def stop(self):
        """
        Disconnects from the server and stops the client loop.
        """
        logging.info("Shutting down...")
        if self.network_interface.connected:
            self.disconnect()
        self.running = False
