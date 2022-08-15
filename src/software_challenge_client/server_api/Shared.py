import src.software_challenge_client.server_api.xtranslate.XTranslateDecorator as XStrDec
from src.software_challenge_client.server_api.protocol import ProtocolPacket
from src.software_challenge_client.server_api.xtranslate.XTranslateInterface import Attribute


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
