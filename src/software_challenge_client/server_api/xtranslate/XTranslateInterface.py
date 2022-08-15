from dataclasses import dataclass
from typing import Any
from xml.etree.ElementTree import Element

from src.software_challenge_client.server_api.protocol import *


class IXObject:

    def setXmlSpecifics(self, element: Element):
        ...

    def getObject(self):
        ...


@dataclass
class Attribute(IXObject):
    caller: ProtocolPacket
    fieldName: str
    fieldValue: Any

    def setXmlSpecifics(self, element: Element):
        if self.fieldValue:
            element.set(self.fieldName, str(self.fieldValue))


@dataclass
class ImplicitArray(IXObject):
    """
    This class is used to define the class attribute as an implicit xml array.
    """
    caller: ProtocolPacket
    fieldName: str
    fieldValue: list[Any]
    itemFieldName: str

    def setXmlSpecifics(self, element: Element):
        """
        This method is to append the implicit array to the XML element. 

        :param element: The element from the element tree that is being built.
        """
        if self.fieldValue:
            for item in self.fieldValue:
                appendix = Element(self.itemFieldName)
                subRoot = Element(item.__class__.__name__)

                for attr, value in item.__dict__.items():
                    value.setXmlSpecifics(element=subRoot)

                appendix.append(subRoot)

                element.append(appendix)
