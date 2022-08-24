import sys

from xsdata.formats.dataclass.context import XmlContext
from xsdata.formats.dataclass.parsers import XmlParser
from xsdata.formats.dataclass.parsers.config import ParserConfig
from xsdata.formats.dataclass.parsers.handlers import XmlEventHandler
from xsdata.formats.dataclass.serializers import XmlSerializer
from xsdata.formats.dataclass.serializers.config import SerializerConfig

from src.SoftwareChallengeClient.api.networking.NetworkInterface import NetworkInterface
from src.SoftwareChallengeClient.api.protocol.Protocol import *
from src.SoftwareChallengeClient.api.sc.Plugin2023 import Team, Move


def customClassFactory(clazz, params: dict):
    if clazz.__name__ == "Data":
        try:
            params.pop("class_binding")
        except KeyError:
            ...
        if params.get("class_value") == "welcomeMessage":
            welcomeMessage = WelcomeMessage(Team(params.get("color")))
            return clazz(class_binding=welcomeMessage, **params)
        elif params.get("class_value") == "memento":
            stateObject = params.get("state")
            return clazz(class_binding=stateObject, **params)
        elif params.get("class_value") == "moveRequest":
            moveRequestObject = MoveRequest()
            return clazz(class_binding=moveRequestObject, **params)
        elif params.get("class_value") == "result":
            resultObject = Result(definition=params.get("definition"), scores=params.get("scores"),
                                  winner=params.get("winner"))
            return clazz(class_binding=resultObject, **params)
        elif params.get("class_value") == "error":
            raise TypeError("Error Class not found!")

    return clazz(**params)


class _XFlux:
    """
    Serialize and deserialize objects to and from XML.
    """

    def __init__(self):
        context = XmlContext()
        deserializeConfig = ParserConfig(class_factory=customClassFactory)
        self.deserializer = XmlParser(handler=XmlEventHandler, context=context, config=deserializeConfig)

        serializeConfig = SerializerConfig(pretty_print=True, xml_declaration=False)
        self.serializer = XmlSerializer(config=serializeConfig)

    def deserializeObject(self, byteStream: bytes) -> ProtocolPacket:
        parsed = self.deserializer.from_bytes(byteStream)
        return parsed

    def serializeObject(self, objectClass: object) -> bytes:
        if isinstance(objectClass, Move):
            from_value = From(x=objectClass.from_value.x, y=objectClass.from_value.y)
            to = To(x=objectClass.to.x, y=objectClass.to.y)
            data = Data(class_value="move", from_value=from_value, to=to)
            return self.serializer.render(data).encode("utf-8")

        return self.serializer.render(objectClass).encode("utf-8")


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
        self.xFlux = _XFlux()
        self.running = False
        self.firstTime = True

    def start(self):
        self.running = True
        self._clientLoop()

    def _clientLoop(self):
        while self.running:
            response = self._receive()
            if isinstance(response, ProtocolPacket):
                if isinstance(response, Left):
                    self.handleDisconnect()
                else:
                    self.onObject(response)
            else:
                raise NotImplementedError("Received object of unknown class.")
        sys.exit()

    def _receive(self) -> ProtocolPacket:
        """
        Gets a receiving byte stream from the server.
        :return: The next object in the stream.
        """
        receiving = self.networkInterface.receive()
        cls = self.xFlux.deserializeObject(receiving)
        return cls

    def send(self, obj: ProtocolPacket):
        """
        Sends an object to the server.
        :param obj: The object to send.
        """
        shipment = self.xFlux.serializeObject(obj)
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
        closeXml = self.xFlux.serializeObject(Close())
        self.networkInterface.send(closeXml)
        self.networkInterface.close()

    def handleDisconnect(self):
        self.closeConnection()
        self.running = False

    def onObject(self, message):
        ...

    def stop(self):
        if self.networkInterface.connected:
            self.closeConnection()
        self.running = False
