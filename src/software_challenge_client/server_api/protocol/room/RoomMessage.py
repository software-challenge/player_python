from typing import Any

import src.software_challenge_client.server_api.xflux.XFluxDecorator as XStrDec
from src.software_challenge_client.server_api.protocol.room.IRoomMessage import ObservableRoomMessage, RoomMessage
from src.software_challenge_client.server_api.sc.api.plugins.IPlugins import IGameState
from src.software_challenge_client.server_api.xflux.XFluxInterface import Attribute


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

    def __init__(self, state: IGameState = None, perspective: Any = None):
        self.__state = state
        self.__perspective = perspective

    @property
    def state(self):
        return self.__state

    @property
    def perspective(self):
        return self.__perspective


@XStrDec.alias(name='moveRequest')
class MoveRequest(RoomMessage):
    """
    Request a Player to send a Move.
    """

    def __init__(self):
        ...
