import src.SoftwareChallengeClient.api.networking.xflux.XFluxDecorator as XStrDec
from src.SoftwareChallengeClient.api.networking.xflux.XFluxInterface import Attribute
from src.SoftwareChallengeClient.api.protocol import ProtocolPacket
from src.SoftwareChallengeClient.api.protocol.room.IRoomMessage import RoomOrchestrationMessage, \
    ObservableRoomMessage
from src.SoftwareChallengeClient.api.sc.Plugin2023 import Team


@XStrDec.alias(name='slotDescriptor')
class SlotDescriptor(ProtocolPacket):

    def __init__(self, displayName: str = None, canTimeout: bool = True, reserved: bool = True):
        self.__displayName = Attribute(caller=self, fieldName="displayName", fieldValue=displayName)
        self.__canTimeout = Attribute(caller=self, fieldName="canTimeout", fieldValue=canTimeout)
        self.__reserved = Attribute(caller=self, fieldName="reserved", fieldValue=reserved)

    def getDisplayName(self):
        return self.__displayName.fieldValue

    def getCanTimeout(self):
        return self.__canTimeout.fieldValue

    def getReserved(self):
        return self.__reserved.fieldValue


@XStrDec.alias(name='result')
class GameResult(RoomOrchestrationMessage, ObservableRoomMessage):
    def __init__(self, definition, scores: dict, winner: Team = None):
        self.definition = definition
        self.scores = scores
        self.winner = winner


@XStrDec.alias(name='left')
class RemovedFromGame(RoomOrchestrationMessage, ObservableRoomMessage):
    def __init__(self, roomId: str = None):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)

    @property
    def roomId(self):
        return self.__roomId.fieldValue
