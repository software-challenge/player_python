from typing import Any

from src.software_challenge_client.server_api.protocol.room.IRoomMessage import RoomMessage


class IMove(RoomMessage):
    ...


class ITeam:
    """
    This represents the team a player is playing for.
    """

    index: int
    name: str
    letter: str

    def opponent(self) -> Any:
        ...


class IGameState(RoomMessage):
    """
    A `GameState` contains all information, that describes the game state at a given time, that is, between two game
    moves.

    This includes:
      - a consecutive turn number (round & turn) and who's turn it is
      - the board
      - the last move made

    The `GameState` is thus the central object through which all essential information of the current game can be
    accessed.

    Therefore, for easier handling, it offers further aids, such as:
      - a method to calculate available moves and to execute moves
      - query whether it should be over

    The game server sends a new copy of the `GameState` to both participating players after each completed move,
    describing the then current state. Information about the course of the game can only be obtained from the
    `GameState` to a limited extent and must therefore be recorded by a game client itself if necessary.

    In addition to the actual information certain partial information can be queried.

    :cvar turn: Current number of moves
    """

    turn: int

    round: int

    currentTeam: ITeam

    isOver: bool

    def getPointsForTeam(self, team: ITeam) -> list[int]:
        ...

    def getPossibleMoves(self) -> list[IMove]:
        ...


class IField(list):
    def isEmpty(self) -> bool:
        ...

    def isOccupied(self) -> bool:
        ...


class IBoard:
    ...
