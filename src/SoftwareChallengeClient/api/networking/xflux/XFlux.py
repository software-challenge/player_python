import logging
import xml.etree.ElementTree as ET
from typing import Any

from src.SoftwareChallengeClient.api.networking.NetworkInterface import NetworkInterface
from src.SoftwareChallengeClient.api.networking.xflux.XFluxInterface import IXmlObject
from src.SoftwareChallengeClient.api.protocol import *
from src.SoftwareChallengeClient.api.protocol.CloseConnection import CloseConnection
from src.SoftwareChallengeClient.api.sc.Plugin2023 import HexBoard, Fishes, Board

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
        root = ET.Element(protocol[obj.__class__])

        for attr, value in obj.__dict__.items():
            if isinstance(value, IXmlObject):
                value.setXmlSpecifics(root)

        return ET.tostring(root)

    def deserialize(self, data: bytes):
        """
        Deserialize xml bytes to an object.
        :param data: The xml bytes to deserialize.
        :return: The deserialized object.
        """
        self.logger.info("Converting XML to object")
        xmlString = data.decode("utf-8").removeprefix("<protocol>\n  ")
        root = ET.fromstring(xmlString)
        cls = protocol[root.attrib["class"] if root.tag == "data" else root.tag]
        # print("Root-Tag:{}, Class:{}".format(root, cls))
        argsList = list(root.attrib.items())
        args = {}
        for key, value in argsList:
            if not key == "class":
                args[key] = value

        if root.text and root.text.isalnum():
            # print("Class:'{}', Value:'{}'".format(cls, root.text))
            return cls(root.text)
        elif cls == Fishes:
            # Get the children of the root element and add them as arguments to the Fishes class.
            children = []
            for child in root:
                children.append(child.text)
            return cls(children[0], children[1])
        elif cls == Board:
            hexBoard = []
            for child in root:
                fields = []
                for children in child:
                    fields.append(self.deserialize(ET.tostring(children)))
                hexBoard.append(fields)
            # Invert the hexBoard to get the correct order.
            hexBoard = [list(x) for x in zip(*hexBoard)]
            return cls(HexBoard(hexBoard))
        else:
            for child in root:
                # Check if the child tag is in the protocol dictionary.
                if child.tag in attributeReference:
                    args[attributeReference[child.tag]] = self.deserialize(ET.tostring(child))
                else:
                    args[child.tag] = self.deserialize(ET.tostring(child))
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
        self.connectToServer()
        self.transposer = _XFlux()
        self.running = False
        self.firstTime = True

    def start(self):
        self.running = True
        self.clientLoop()

    def clientLoop(self):
        while self.running:
            response = self.receive()

            if isinstance(response, ProtocolPacket):
                if isinstance(response, CloseConnection):
                    self.handleDisconnect()
                else:
                    self.onObject(response)
            else:
                raise NotImplementedError("Received object of unknown class.")

    def receive(self) -> ProtocolPacket:
        """
        Gets a receiving byte stream from the server.
        :return: The next object in the stream.
        """
        receiving = self.networkInterface.receive()
        print(receiving.decode("utf-8"))
        cls = self.transposer.deserialize(receiving)
        return cls

    def send(self, obj: ProtocolPacket):
        """
        Sends an object to the server.
        :param obj: The object to send.
        """
        shipment = self.transposer.serialize(obj)
        if self.firstTime:
            shipment = "<protocol>".encode("utf-8") + shipment
            self.firstTime = False
        print(shipment.decode("utf-8"))
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

    def handleDisconnect(self):
        ...

    def onObject(self, message):
        ...

    def stop(self):
        if self.networkInterface.connected:
            self.closeConnection()
        self.running = False
