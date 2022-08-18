from dataclasses import dataclass
from typing import Any
from xml.etree.ElementTree import Element


class IXmlObject:

    def setXmlSpecifics(self, element: Element):
        ...


@dataclass
class Traverse(IXmlObject):
    """
    Traverses a object until it finds an Attribute or ImplicitArray.
    """
    caller: Any
    fieldValue: Any

    def setXmlSpecifics(self, element: Element):
        for attr, value in self.fieldValue.__dict__.items():
            value.setXmlSpecifics(element)


@dataclass
class Attribute(IXmlObject):
    caller: Any
    fieldName: str
    fieldValue: Any

    def setXmlSpecifics(self, element: Element):
        if self.fieldValue:
            element.set(self.fieldName, str(self.fieldValue))


@dataclass
class ImplicitArray(IXmlObject):
    """
    Used to define the class attribute as an implicit xml array.
    """
    caller: Any
    fieldName: str
    fieldValue: list[Any]
    itemFieldName: str = None

    def setXmlSpecifics(self, element: Element):
        """
        Appends the implicit array to the XML element.

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
