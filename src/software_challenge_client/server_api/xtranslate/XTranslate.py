import logging
import xml.etree.ElementTree as ET
from typing import Any

from src.software_challenge_client.server_api.networking.NetworkInterface import NetworkInterface
from src.software_challenge_client.server_api.protocol import *

protocol: dict[Any, Any] = protocolClasses


class _XParse:
    def __init__(self):
        global protocol
        protocol = self._dictionary(ProtocolPacket)
        self.logger = logging.getLogger(__name__)

    def readObject(self, obj: ProtocolPacket) -> bytes:
        self.logger.info("Converting object to XML")
        root = ET.Element(obj.__class__.__name__)
        # For each variable in the object
        for attr, value in obj.__dict__.items():
            value.setXmlSpecifics(root)

        return ET.tostring(root)

    def readXML(self, xString: str) -> ProtocolPacket:
        self.logger.info("Converting XML to object")
        xString = xString.removeprefix("<protocol>\n  ")
        root = ET.fromstring(xString)
        obj = protocol[root.tag]()
        for child in root:
            setattr(obj, child.tag, child.text)
        return obj

    def _getSubClasses(self, cls):
        return set(cls.__subclasses__()).union(
            [s for c in cls.__subclasses__() for s in self._getSubClasses(c)])

    def _dictionary(self, cls):
        dictionary = {}
        for sub in self._getSubClasses(cls):
            dictionary[sub.__name__] = sub
        return dict(dictionary)


class XTranslate:
    def __init__(self, networkInterface: NetworkInterface):
        self.networkInterface = networkInterface
        self.parser = _XParse()

    def inStream(self) -> ProtocolPacket:
        receiving = self.networkInterface.receive()
        return self.parser.readXML(receiving)

    def outStream(self, obj: ProtocolPacket):
        shipment = self.parser.readObject(obj)
        shipment = "<protocol>" + shipment.decode("utf-8")
        self.networkInterface.send(shipment)


logging.basicConfig(level=logging.INFO)
# network = NetworkInterface("localhost", 13050)
# network.connect()
# xmlStream = XStrDec(network)
prepare = JoinGameRequest()
# Print prepare object with attributes
# print(prepare)
# print(prepare)
# print(prepare.__class__)
# print(protocol)
parse = _XParse()
string = parse.readObject(prepare)

print(string)
