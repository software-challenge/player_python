import src.software_challenge_client.server_api.networking.xflux.XFluxDecorator as XStrDec
from src.software_challenge_client.server_api.networking.xflux.XFluxInterface import Attribute, Traverse
from src.software_challenge_client.server_api.protocol import ProtocolPacket
from src.software_challenge_client.server_api.protocol.room.IRoomMessage import RoomMessage


@XStrDec.alias(name='room')
class RoomPacket(ProtocolPacket):
    """
    Used to send a RoomMessage to a room.
    """

    def __init__(self, roomId: str = None, data: RoomMessage = None):
        self.__roomId = Attribute(caller=self, fieldName="roomId", fieldValue=roomId)
        self.__data = Traverse(self, data)

    def setRoomId(self, roomId: str):
        self.__roomId.fieldValue = roomId

    def getRoomId(self):
        return self.__roomId.fieldValue

    def setData(self, data: RoomMessage):
        self.__data.fieldValue = data

    def getData(self):
        return self.__data.fieldValue
