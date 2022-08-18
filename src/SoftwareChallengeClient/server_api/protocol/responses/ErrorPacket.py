import src.SoftwareChallengeClient.server_api.networking.xflux.XFluxDecorator as XStrDec
from src.SoftwareChallengeClient.server_api.networking.xflux.XFluxInterface import Attribute
from src.SoftwareChallengeClient.server_api.protocol.ProtocolPacket import ResponsePacket, ProtocolPacket


@XStrDec.alias(name="errorpacket")
class ErrorPacket(ResponsePacket):
    """
    Response to an erroneous packet, including an error message.
    """
    __originalRequest: ProtocolPacket

    def __init__(self, originalRequest: ProtocolPacket = None, message: str = None):
        self.__originalRequest = originalRequest
        self.__message = Attribute(caller=self, fieldName="message", fieldValue=message)

    def getOriginalRequest(self):
        return self.__originalRequest

    def getMessage(self):
        return self.__message.fieldValue

    def setMessage(self, message: str):
        self.__message.fieldValue = message

    def setOriginalRequest(self, originalRequest: ProtocolPacket):
        self.__originalRequest = originalRequest
