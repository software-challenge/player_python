"""
This module handels the communication with the api and the students logic.
"""
import logging
import time

from socha.api.networking._xflux import _XFluxClient
from socha.api.plugin import penguins
from socha.api.plugin.penguins import Field, GameState, Move, Coordinate
from socha.api.protocol.protocol import Slot, State, AdminLobbyRequest, Authenticate, Board, Cancel, Close, Data, \
    Definition, Entry, Error, Fishes, Fragment, From, Join, Joined, JoinedGameRoom, JoinPrepared, JoinRoom, LastMove, \
    Left, ListType, LobbyRequest, Winner, WelcomeMessage, Union, To, Team, Step, Scores, Score, \
    RoomOrchestrationMessage, RoomMessage, Room, Result, ResponsePacket, ProtocolPacket, Protocol, Prepare, Player, \
    Pause, OriginalMessage, Optional, Observe, MoveRequest, ObservableRoomMessage


def _convertBoard(protocolBoard: Board) -> penguins.Board:
    """
    Converts a protocol Board to a usable gam board for using in the logic.
    :rtype: object
    """
    boardList: list[list[Field]] = []
    for y, row in enumerate(protocolBoard.list_value):
        rowList: list[Field] = []
        for x, fieldsValue in enumerate(row.field_value):
            fieldCoordinate = Coordinate(x, y, is_double=False).get_double_hex()
            rowList.append(Field(coordinate=fieldCoordinate, field=fieldsValue))
        boardList.append(rowList)
    return penguins.Board(boardList)


class IClientHandler:
    def calculate_move(self) -> Move:
        """
        Calculates a move that the logic wants the server to perform in the game room.
        """

    def on_update(self, state: GameState):
        """
        If the server send a update on the current state of the game this method is called.
        :param state: The current state that server sent.
        """

    def on_game_over(self, roomMessage: Result): ...

    def on_error(self, logMessage: str):
        """
        If error occurs,
        for instance when the logic sent a move that is not rule conform,
        the server will send an error message and closes the connection.
        If this happens, this method is called.

        :param logMessage: The message, that server sent.
        """

    def on_room_message(self, data): ...

    def on_game_prepared(self, message): ...

    def on_game_joined(self, room_id): ...

    def on_game_observed(self, message): ...


class _PlayerClient(_XFluxClient):
    """
    The PlayerClient handles all incoming and outgoing objects accordingly to their types.
    """

    def __init__(self, host: str, port: int, handler: IClientHandler, keep_alive: bool):
        super().__init__(host, port)
        self.game_handler = handler
        self.keep_alive = keep_alive

    def authenticate(self, password: str, consumer):
        ...

    def observe_room(self, room_id: str, observer):
        ...

    def join_game(self, game_type: str = None):
        super().send(Join())

    def join_game_room(self, room_id: str):
        super().send(JoinRoom(room_id=room_id))

    def join_game_with_reservation(self, reservation: str):
        super().send(JoinPrepared(reservation_code=reservation))

    def send_message_to_room(self, room_id: str, message):
        super().send(Room(room_id=room_id, data=message))

    def on_object(self, message):
        if isinstance(message, Room):
            room_id: str = message.room_id
            data = message.data.class_binding
            if isinstance(data, MoveRequest):
                start_time = time.time()
                response = self.game_handler.calculate_move()
                logging.info(f"Sent {response} after {time.time() - start_time} seconds.")
                if response:
                    from_value = None
                    to = To(x=response.to_value.x, y=response.to_value.y)
                    if response.from_value:
                        from_value = From(x=response.from_value.x, y=response.from_value.y)
                    response = Data(class_value="move", from_value=from_value, to=to)
                    self.send_message_to_room(room_id, response)
            if isinstance(data, ObservableRoomMessage):
                # TODO Set observer data
                if isinstance(data, State):
                    game_state = GameState(turn=data.turn, start_team=Team(data.start_team),
                                           board=_convertBoard(data.board), last_move=data.last_move,
                                           fishes=penguins.Fishes(data.fishes.int_value[0], data.fishes.int_value[1]))
                    self.game_handler.on_update(game_state)
                elif isinstance(data, Result):
                    self.game_handler.on_game_over(data)
                elif isinstance(data, Error):
                    # TODO Logger
                    self.game_handler.on_error(data.message)
            if isinstance(data, Error):
                logging.error(data.message)
                self.game_handler.on_error(data.message)
            else:
                self.game_handler.on_room_message(data)
        elif isinstance(message, Joined):
            self.game_handler.on_game_joined(room_id=message.room_id)
