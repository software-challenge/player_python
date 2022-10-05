"""
This module handels the communication with the api and the students logic.
"""
import logging
import time
from typing import List, Union

from python.socha.socha import GameState, Move, Field, CartesianCoordinate, Team, Score, TeamEnum, Progress, Penguin, \
    HexCoordinate

from python.socha import socha
from python.socha.api.networking._xflux import _XFluxClient
from python.socha.api.protocol.protocol import State, Board, Data, \
    Error, From, Join, Joined, JoinPrepared, JoinRoom, To, Room, Result, MoveRequest, ObservableRoomMessage, \
    WelcomeMessage
from python.socha.api.protocol.room_message import RoomOrchestrationMessage


def _convertBoard(protocolBoard: Board) -> socha.Board:
    """
    Converts a protocol Board to a usable gam board for using in the logic.
    :rtype: object
    """
    boardList = []
    for y, row in enumerate(protocolBoard.list_value):
        rowList: List[Field] = []
        for x, fieldsValue in enumerate(row.field_value):
            fieldCoordinate = CartesianCoordinate(x, y).to_hex()
            if not isinstance(fieldsValue, int):
                fieldTeamEnum = TeamEnum.ONE if fieldsValue == 'ONE' else TeamEnum.TWO
                penguin: Penguin = Penguin(position=fieldCoordinate, team=fieldTeamEnum)
                rowList.append(Field(coordinate=fieldCoordinate, penguin=penguin, fish=0))
            else:
                rowList.append(Field(coordinate=fieldCoordinate, penguin=None, fish=int(fieldsValue)))
        boardList.append(rowList)
    return socha.Board(boardList)


class IClientHandler:
    history: List[Union[GameState, Error, Result]] = []

    def calculate_move(self) -> Move:
        """
        Calculates a move that the logic wants the server to perform in the game room.
        """

    def on_update(self, state: GameState):
        """
        If the server send a update on the current state of the game this method is called.
        :param state: The current state that server sent.
        """

    def on_game_over(self, roomMessage: Result):
        """
        If the game has ended the server will send a result message.
        This method will called if this happens.

        :param roomMessage: The Result the server has sent.
        """

    def on_error(self, logMessage: str):
        """
        If error occurs,
        for instance when the logic sent a move that is not rule conform,
        the server will send an error message and closes the connection.
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


class _PlayerClient(_XFluxClient):
    """
    The PlayerClient handles all incoming and outgoing objects accordingly to their types.
    """
    welcome_message: socha.WelcomeMessage

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
            if isinstance(data, RoomOrchestrationMessage):
                if isinstance(data, WelcomeMessage):
                    self.welcome_message = socha.WelcomeMessage(data.team.name)
            if isinstance(data, MoveRequest):
                start_time = time.time()
                response = self.game_handler.calculate_move()
                logging.info(f"Sent {response} after {time.time() - start_time} seconds.")
                if response:
                    from_value = None
                    to = To(x=response.to.x, y=response.to.y)
                    if response._from:
                        from_value = From(x=response._from.x, y=response._from.y)
                    response = Data(class_value="move", from_value=from_value, to=to)
                    self.send_message_to_room(room_id, response)
            if isinstance(data, ObservableRoomMessage):
                # TODO Set observer data
                if isinstance(data, State):
                    board: socha.Board = _convertBoard(data.board)
                    logging.info(board)
                    penguins_one = board.get_team_penguins(TeamEnum.ONE)
                    fish_one: int = data.fishes.int_value[0]
                    team_one: Team = Team(name=TeamEnum.ONE, penguins=penguins_one, fish=fish_one)
                    penguins_two = board.get_team_penguins(TeamEnum.TWO)
                    fish_two: int = data.fishes.int_value[1]
                    team_two: Team = Team(name=TeamEnum.TWO, penguins=penguins_two, fish=fish_two)
                    game_state = GameState(
                        welcome_message=self.welcome_message,
                        start_team=team_one if data.start_team == "ONE" else team_two,
                        board=board,
                        last_move=None if data.last_move is None else Move(
                            _from=None if data.last_move.from_value is None else
                            HexCoordinate(x=data.last_move.from_value.x,
                                          y=data.last_move.from_value.y),
                            to=HexCoordinate(x=data.last_move.to.x,
                                             y=data.last_move.to.y),
                            team=team_one.name),
                        round=Progress(round=data.turn // 2, turn=data.turn),
                        score=Score(
                            team_one=team_one,
                            team_two=team_two
                        )
                    )
                    self.game_handler.history.append(game_state)
                    self.game_handler.on_update(game_state)
                elif isinstance(data, Result):
                    self.game_handler.history.append(data)
                    self.game_handler.on_game_over(data)
            if isinstance(data, Error):
                logging.error(data.message)
                self.game_handler.history.append(data)
                self.game_handler.on_error(data.message)
            else:
                self.game_handler.on_room_message(data)
        elif isinstance(message, Joined):
            self.game_handler.on_game_joined(room_id=message.room_id)
