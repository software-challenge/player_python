import xml.etree.ElementTree as ET

from src.SoftwareChallengeClient.api.Shared import protocolClasses, attributeReference
from src.SoftwareChallengeClient.api.networking.NetworkInterface import NetworkInterface
from src.SoftwareChallengeClient.api.networking.xflux.XFluxInterface import IXmlObject
from src.SoftwareChallengeClient.api.protocol import *
from src.SoftwareChallengeClient.api.protocol.CloseConnection import CloseConnection
from src.SoftwareChallengeClient.api.sc.Plugin2023 import HexBoard, Fishes, Board


class _XFlux:
    """
    Serialize and deserialize objects to and from XML.
    """

    @staticmethod
    def serialize(obj: ProtocolPacket) -> bytes:
        """
        Serialize an object to XML.
        :param obj: The object to serialize.
        :return: The serialized object as xml bytes.
        """
        root = ET.Element(protocolClasses[obj.__class__])

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
        xmlString = data.decode("utf-8").removeprefix("<protocol>\n  ")
        root = ET.fromstring(xmlString)
        cls = protocolClasses[root.attrib["class"] if root.tag == "data" else root.tag]
        argsList = list(root.attrib.items())
        args = {}
        for key, value in argsList:
            if not key == "class":
                args[key] = value

        if root.text and root.text.isalnum():
            return cls(root.text)
        elif cls == Fishes:
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
            hexBoard = [list(x) for x in zip(*hexBoard)]
            return cls(HexBoard(hexBoard))
        else:
            for child in root:
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
