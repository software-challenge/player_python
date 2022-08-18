import src.software_challenge_client.server_api.networking.xflux.XFluxDecorator as XStrDec
from src.software_challenge_client.server_api.networking.xflux.XFluxInterface import Attribute
from src.software_challenge_client.server_api.protocol.room.IRoomMessage import RoomMessage, RoomOrchestrationMessage, \
    ObservableRoomMessage


@XStrDec.alias(name='error')
class ErrorMessage(RoomOrchestrationMessage, ObservableRoomMessage):
    """
    Response to an erroneous message, including an error message.
    """

    def __init__(self, originalMessage: RoomMessage = None, message: str = None):
        self.__originalMessage = originalMessage
        self.__message = Attribute(caller=self, fieldName="message", fieldValue=message)

    def getLogMessage(self):
        return "{} caused an error: {}".format(str(self.__originalMessage), self.__message.fieldValue)

    @property
    def originalMessage(self):
        return self.__originalMessage

    @property
    def message(self):
        return self.__message.fieldValue
