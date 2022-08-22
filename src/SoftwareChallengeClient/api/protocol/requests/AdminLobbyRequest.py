import src.SoftwareChallengeClient.api.networking.xflux.XFluxDecorator as XStrDec
from src.SoftwareChallengeClient.api.networking.xflux.XFluxInterface import Attribute, ImplicitArray
from src.SoftwareChallengeClient.api.protocol.ProtocolPacket import AdminLobbyRequest
from src.SoftwareChallengeClient.api.protocol.Shared import SlotDescriptor


@XStrDec.alias(name='authenticate')
class AuthenticateRequest(AdminLobbyRequest):
    """
    Authenticates a client as administrator to send AdminLobbyRequest`s. \n
    *Is not answered if successful.*
    """

    def __init__(self, password: str = None):
        self.__password = Attribute(caller=self, fieldName="password", fieldValue=password)

    def getPassword(self):
        return self.__password.fieldValue

    def setPassword(self, password: str):
        self.__password.fieldValue = password


@XStrDec.alias(name='cancel')
class CancelRequest(AdminLobbyRequest):
    """
    Deletes the GameRoom and cancels the Game within.
    """

    def __init__(self, roomId: str = None):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)

    def getRoomId(self):
        return self.__roomId.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId


@XStrDec.alias(name='observe')
class ObservationRequest(AdminLobbyRequest):
    """
    Request by administrative client to observe a gameRoom specified by given roomId.
    """

    def __init__(self, roomId: str = None):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)

    def getRoomId(self):
        return self.__roomId.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId


@XStrDec.alias(name='pause')
class PauseGameRequest(AdminLobbyRequest):
    """
    Request by administrative client to pause or unpause a game specified by given roomId.

    A game will only be paused immediately if there is no pending MoveRequest,
    otherwise the game will be paused next turn.

    When the game is paused no GameState or MoveRequest will be sent to the players (and all other observers)
    until an AdminClient sends a StepRequest or resumes the game.
    """

    def __init__(self, roomId: str = None):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)

    def getRoomId(self):
        return self.__roomId.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId


@XStrDec.alias(name='prepare')
class PrepareGameRequest(AdminLobbyRequest):
    """
     Prepare a game of __gameType with slots according to slotDescriptors.
    """

    def __init__(self, gameType: str = None, slotDescriptors=None, pause: str = None):
        """
        Create a prepared game with descriptors for each player.

        :param gameType: Type of the game (plugin id).
        :param slotDescriptors: The player descriptors default to "Player1" and "Player2".
        :param pause: Whether the game should start paused.
        """
        if slotDescriptors is None:
            slotDescriptors = [SlotDescriptor(displayName="Player1"), SlotDescriptor(displayName="Player1")]

        self.__gameType = Attribute(caller=self, fieldName="gameType", fieldValue=gameType)
        self.__slotDescriptors = ImplicitArray(caller=self, fieldName="slotDescriptors",
                                               fieldValue=slotDescriptors, itemFieldName="slot")
        self.__pause = Attribute(caller=self, fieldName="pause", fieldValue=pause)

    def __eq__(self, other) -> bool:
        return self.getGameType() == other.getGameType() and self.getSlotDescriptors() == other.getSlotDescriptors() \
               and self.getPause() == other.getPause()

    def __hash__(self) -> int:
        return (hash(self.getGameType()) * 31 + hash(self.getSlotDescriptors())) * 31 + hash(self.getPause())

    def getGameType(self) -> str:
        return self.__gameType.fieldValue

    def getSlotDescriptors(self) -> list[SlotDescriptor]:
        return self.__slotDescriptors.fieldValue

    def getPause(self) -> bool:
        return self.__pause.fieldValue

    def setGameType(self, gameType: str):
        self.__gameType.fieldValue = gameType

    def setSlotDescriptors(self, slotDescriptors: list[SlotDescriptor]):
        self.__slotDescriptors.fieldValue = slotDescriptors

    def setPause(self, pause: str):
        self.__pause.fieldValue = pause


@XStrDec.alias(name='step')
class StepRequest(AdminLobbyRequest):
    """
    Request by administrative client to send a MoveRequest to the current player.
    Only works for paused games.
    """

    def __init__(self, roomId: str = None, forced: bool = False):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)
        self.__forced = Attribute(caller=self, fieldName="forced", fieldValue=forced)

    def getRoomId(self):
        return self.__roomId.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId

    def getForced(self):
        return self.__forced.fieldValue

    def setForced(self, forced: bool):
        self.__forced.fieldValue = forced
