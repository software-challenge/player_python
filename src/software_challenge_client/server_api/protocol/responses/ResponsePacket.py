import src.software_challenge_client.server_api.xflux.XFluxDecorator as XStrDec
from src.software_challenge_client.server_api.protocol.IProtocolPacket import ResponsePacket
from src.software_challenge_client.server_api.xflux.XFluxInterface import Attribute, ImplicitArray


@XStrDec.alias(name="prepared")
class GamePreparedResponse(ResponsePacket):
    """
    Response to sc.protocol.requests.PrepareGameRequest.
    """

    def __init__(self, roomId: str = None, reservations: list[str] = None):
        """
        :param roomId: The id of the room that this response is coming from.
        :param reservations: The reservations for the reserved slots.
        """
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)
        self.__reservations = ImplicitArray(caller=self, fieldName="reservations", fieldValue=reservations,
                                            itemFieldName="reservations")

    def getRoomId(self):
        return self.__roomId.fieldValue

    def getReservations(self):
        return self.__reservations.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId

    def setReservations(self, reservations: list[str]):
        self.__reservations.fieldValue = reservations


@XStrDec.alias(name="joined")
class JoinedRoomResponse(ResponsePacket):
    """
    Response to client who successfully joined a game.
    """

    def __init__(self, roomId: str = None):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)

    def getRoomId(self):
        return self.__roomId.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId


@XStrDec.alias(name="observed")
class ObservationResponse(ResponsePacket):
    """
    Sent to client as response to successfully joining a GameRoom as Observer.
    """

    def __init__(self, roomId: str = None):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)

    def getRoomId(self):
        return self.__roomId.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId


@XStrDec.alias(name="joinedGameRoom")
class RoomJoinedResponse(ResponsePacket):
    """
    Originally named **RoomWasJoinedEvent**! \n
    Sent to all administrative clients after a player joined a GameRoom via a JoinRoomRequest.
    """

    def __init__(self, roomId: str = None, playerCount: int = None):
        """
        :param roomId: The id of the room that this response is coming from.
        :param playerCount: The number of players in the room after the join.
        """
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)
        self.__playerCount = Attribute(caller=self, fieldName="playerCount", fieldValue=playerCount)

    def getRoomId(self):
        return self.__roomId.fieldValue

    def getPlayerCount(self):
        return self.__playerCount.fieldValue

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId

    def setPlayerCount(self, playerCount: int):
        self.__playerCount.fieldValue = playerCount
