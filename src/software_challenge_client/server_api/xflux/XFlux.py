import logging
import xml.etree.ElementTree as ET
from typing import Any

from src.software_challenge_client.server_api.networking.NetworkInterface import NetworkInterface
from src.software_challenge_client.server_api.protocol import *
from src.software_challenge_client.server_api.protocol.CloseConnection import CloseConnection
from src.software_challenge_client.server_api.xflux.XFluxInterface import IXmlObject

protocol: dict[Any, Any] = protocolClasses


class _XFlux:
    """
    Serialize and deserialize objects to and from XML.
    """

    def __init__(self):
        global protocol
        self.logger = logging.getLogger(__name__)

    def serialize(self, obj: ProtocolPacket) -> bytes:
        """
        Serialize an object to XML.
        :param obj: The object to serialize.
        :return: The serialized object as xml bytes.
        """
        self.logger.info("Converting object to XML")
        root = ET.Element(obj.__class__.__name__)

        for attr, value in obj.__dict__.items():
            if value is IXmlObject:
                value.setXmlSpecifics(root)

        return ET.tostring(root)

    def deserialize(self, data: bytes) -> ProtocolPacket:
        """
        Deserialize xml bytes to an object.
        :param data: The xml bytes to deserialize.
        :return: The deserialized object.
        """
        self.logger.info("Converting XML to object")
        xmlString = data.decode("utf-8").removeprefix("<protocol>\n  ")
        root = ET.fromstring(xmlString)
        cls = protocol[root.tag]
        args = dict(root.attrib.items())

        return cls(**args)


class XFluxClient:
    """
    Streams data from and to the server.
    """

    def __init__(self, host: str, port: int):
        """
        :param host: Host of the server.
        :param port: Port of the server.
        """
        self.networkInterface = NetworkInterface(host, port)
        self.transposer = _XFlux()

    def inStream(self) -> ProtocolPacket:
        """
        Gets a receiving byte stream from the server.
        :return: The next object in the stream.
        """
        receiving = self.networkInterface.receive()
        return self.transposer.deserialize(receiving)

    def outStream(self, obj: ProtocolPacket):
        """
        Sends an object to the server.
        :param obj: The object to send.
        """
        shipment = self.transposer.serialize(obj)
        shipment = "<protocol>".encode("utf-8") + shipment
        self.networkInterface.send(shipment)

    def connectToServer(self):
        """
        Creates a TCP connection with the server.
        """
        self.networkInterface.connect()

    def closeConnection(self):
        """
        Sends a closing xml to the server and closes the connection afterwards.
        """
        closeXml = self.transposer.serialize(CloseConnection())
        self.networkInterface.send(closeXml)
        self.networkInterface.close()
