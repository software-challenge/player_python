import src.software_challenge_client.server_api.xtranslate.XTranslateDecorator as XStrDec
from src.software_challenge_client.server_api.protocol.ProtocolPacket import LobbyRequest
from src.software_challenge_client.server_api.xtranslate.XTranslateInterface import Attribute


@XStrDec.alias(name='join')
class JoinGameRequest(LobbyRequest):
    """
    Join a game by gameType.
    Creates a new gameRoom if no open gameRoom of the specified gameType exists.
    """

    def __init__(self, gameType: str = None):
        self.gameType = Attribute(caller=self, fieldName="gameType", fieldValue=gameType)

    def getGameType(self):
        return self.gameType.fieldValue

    def setGameType(self, gameType: str):
        self.gameType.fieldValue = gameType


@XStrDec.alias(name='joinPrepared')
class JoinPreparedRoomRequest(LobbyRequest):
    """
    Join a prepared game by reservation.
    The code is handed out by the administrative client that created the game via a PrepareGameRequest.
    """

    def __init__(self, reservationCode: str = None):
        self.reservationCode = Attribute(caller=self, fieldName="reservationCode", fieldValue=reservationCode)

    def getReservationCode(self):
        return self.reservationCode.fieldValue

    def setReservationCode(self, reservationCode: str):
        self.reservationCode.fieldValue = reservationCode


@XStrDec.alias(name='joinRoom')
class JoinRoomRequest(LobbyRequest):
    """
    Join a prepared GameRoom without reservation.
    """

    def __init__(self, roomId: str = None):
        self.roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)

    def getRoomId(self):
        return self.roomId.fieldValue

    def setRoomId(self, roomId: str):
        self.roomId.fieldValue = roomId
