"""
Here are all incoming byte streams and all outgoing protocol objects handelt.
"""
import logging
import sys

from xsdata.formats.dataclass.context import XmlContext
from xsdata.formats.dataclass.parsers import XmlParser
from xsdata.formats.dataclass.parsers.config import ParserConfig
from xsdata.formats.dataclass.parsers.handlers import XmlEventHandler
from xsdata.formats.dataclass.serializers import XmlSerializer
from xsdata.formats.dataclass.serializers.config import SerializerConfig

from socha.api.networking._network_interface import _NetworkInterface
from socha.api.plugin.penguins import Move
from socha.api.protocol.protocol import *


def customClassFactory(clazz, params: dict):
    if clazz.__name__ == "Data":
        try:
            params.pop("class_binding")
        except KeyError:
            ...
        if params.get("class_value") == "welcomeMessage":
            welcome_message = WelcomeMessage(Team(params.get("color")))
            return clazz(class_binding=welcome_message, **params)
        elif params.get("class_value") == "memento":
            state_object = params.get("state")
            return clazz(class_binding=state_object, **params)
        elif params.get("class_value") == "moveRequest":
            move_request_object = MoveRequest()
            return clazz(class_binding=move_request_object, **params)
        elif params.get("class_value") == "result":
            result_object = Result(definition=params.get("definition"), scores=params.get("scores"),
                                   winner=params.get("winner"))
            return clazz(class_binding=result_object, **params)
        elif params.get("class_value") == "error":
            error_object = Error(message=params.get("message"), originalMessage=params.get("original_message"))
            return clazz(class_binding=error_object, **params)

    return clazz(**params)


class _XFlux:
    """
    Serialize and deserialize objects to and from XML.
    """

    def __init__(self):
        context = XmlContext()
        deserialize_config = ParserConfig(class_factory=customClassFactory)
        self.deserializer = XmlParser(handler=XmlEventHandler, context=context, config=deserialize_config)

        serialize_config = SerializerConfig(pretty_print=True, xml_declaration=False)
        self.serializer = XmlSerializer(config=serialize_config)

    def deserialize_object(self, byteStream: bytes) -> ProtocolPacket:
        """
        Deserialize a xml byte stream to a ProtocolPacket.
        :param byteStream: The byte stream to deserialize.
        :return: The deserialized ProtocolPacket child.
        """
        parsed = self.deserializer.from_bytes(byteStream)
        return parsed

    def serialize_object(self, object_class: object) -> bytes:
        """
        Serialize a ProtocolPacket child to a xml byte stream.
        :param object_class: The ProtocolPacket child to serialize.
        :return: The serialized byte stream.
        """
        if isinstance(object_class, Move):
            from_value = From(x=object_class.from_value.x, y=object_class.from_value.y)
            to_value = To(x=object_class.to_value.x, y=object_class.to_value.y)
            data = Data(class_value="move", from_value=from_value, to=to_value)
            return self.serializer.render(data).encode("utf-8")

        return self.serializer.render(object_class).encode("utf-8")


class _XFluxClient:
    """
    Streams data from and to the server.
    """

    def __init__(self, host: str, port: int):
        """
        :param host: Host of the server.
        :param port: Port of the server.
        """
        self.network_interface = _NetworkInterface(host, port)
        self.connect_to_server()
        self.x_flux = _XFlux()
        self.running = False
        self.first_time = True

    def start(self):
        """
        Starts the client loop.
        """
        self.running = True
        self._client_loop()

    def _client_loop(self):
        """
        The client loop.
        This is the main loop,
        where the client waits for messages from the server
        and handles them accordingly.
        """
        while self.running:
            response = self._receive()
            if isinstance(response, ProtocolPacket):
                if isinstance(response, Left):
                    logging.info("The server left. Shutting down...")
                    self.handle_disconnect()
                else:
                    logging.info(f"Received new object: {response}")
                    self.on_object(response)
            elif self.running:
                logging.error(f"Received object of unknown class: {response}")
                raise NotImplementedError("Received object of unknown class.")
        logging.info("Done.")
        sys.exit()

    def _receive(self):
        """
        Gets a receiving byte stream from the server.
        :return: The next object in the stream.
        """
        try:
            receiving = self.network_interface.receive()
            cls = self.x_flux.deserialize_object(receiving)
            return cls
        except OSError:
            logging.error("Shutting down abnormally...")
            self.running = False

    def send(self, obj: ProtocolPacket):
        """
        Sends an object to the server.
        :param obj: The object to send.
        """
        shipment = self.x_flux.serialize_object(obj)
        if self.first_time:
            shipment = "<protocol>".encode("utf-8") + shipment
            self.first_time = False
        self.network_interface.send(shipment)

    def connect_to_server(self):
        """
        Creates a TCP connection with the server.
        """
        self.network_interface.connect()

    def close_connection(self):
        """
        Sends a closing xml to the server and closes the connection afterwards.
        """
        close_xml = self.x_flux.serialize_object(Close())
        self.network_interface.send(close_xml)
        self.network_interface.close()

    def handle_disconnect(self):
        """
        Closes the connection and stops the client loop.
        """
        self.close_connection()
        self.running = False

    def on_object(self, message):
        """
        Handles an object received from the server.
        :param message: The object to handle.
        """

    def stop(self):
        """
        Disconnects from the server and stops the client loop.
        """
        if self.network_interface.connected:
            self.close_connection()
        self.running = False
