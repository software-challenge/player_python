from typing import Any

import src.SoftwareChallengeClient.api.networking.xflux.XFluxDecorator as XStrDec
from src.SoftwareChallengeClient.api.networking.xflux.XFluxInterface import Attribute, Traverse
from src.SoftwareChallengeClient.api.protocol.room.IRoomMessage import ObservableRoomMessage, RoomMessage, \
    RoomOrchestrationMessage
from src.SoftwareChallengeClient.api.sc.Plugin2023 import Team, GameState


@XStrDec.alias(name='welcomeMessage')
class WelcomeMessage(RoomOrchestrationMessage):
    """
    Message sent to a client at the beginning of a game to tell him his player color.
    """

    def __init__(self, color: str = None):
        self.__color = Attribute(caller=self, fieldName="color", fieldValue=color)
        self.team: Team = Team(self.color)

    @property
    def color(self):
        return self.__color.fieldValue


@XStrDec.alias(name='paused')
class GamePaused(ObservableRoomMessage):
    """
    Indicates to observers that the game has been (un)paused.
    """

    def __init__(self, paused: bool = None):
        self.__paused = Attribute(caller=self, fieldName="paused", fieldValue=paused)

    @property
    def paused(self):
        return self.__paused.fieldValue


@XStrDec.alias(name='memento')
class MementoMessage(ObservableRoomMessage):
    """
    Sent to update the current state and potentially also a new perspective.
    """

    def __init__(self, state: GameState = None, perspective: Any = None):
        self.__state = Traverse(self, state)
        self.__perspective = perspective

    @property
    def state(self):
        return self.__state.fieldValue

    @property
    def perspective(self):
        return self.__perspective


@XStrDec.alias(name='moveRequest')
class MoveRequest(RoomMessage):
    """
    Request a Player to send a Move.
    """

    def __init__(self): ...
