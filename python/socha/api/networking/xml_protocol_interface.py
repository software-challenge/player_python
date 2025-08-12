"""
Here are all incoming byte streams and all outgoing protocol objects handled.
"""

import contextlib
import logging
from typing import Any, Callable, Iterator

from socha.api.networking.utils import map_string_to_direction
from socha import _socha
from socha.api.networking.network_socket import NetworkSocket
from socha.api.protocol.protocol import (
    Close,
    Error,
    MoveRequest,
    Result,
    WelcomeMessage,
)
from socha.api.protocol.protocol_packet import ProtocolPacket
from xsdata.formats.dataclass.context import XmlContext
from xsdata.formats.dataclass.parsers import XmlParser
from xsdata.formats.dataclass.parsers.config import ParserConfig
from xsdata.formats.dataclass.parsers.handlers import XmlEventHandler
from xsdata.formats.dataclass.serializers import XmlSerializer
from xsdata.formats.dataclass.serializers.config import SerializerConfig

from socha.api.protocol.protocol import Data, LastMove


def map_object(data: Data , params: dict):

    try:
        params.pop('class_binding')
    except KeyError:
        ...
    if params.get('class_value') == 'welcomeMessage':
        welcome_message = WelcomeMessage(
            _socha.TeamEnum.One if params.get('color') == 'ONE' else _socha.TeamEnum.Two
        )
        return data(class_binding=welcome_message, **params)
    elif params.get('class_value') == 'memento':
        state_object = params.get('state')
        return data(class_binding=state_object, **params)
    elif params.get('class_value') == 'moveRequest':
        move_request_object = MoveRequest()
        return data(class_binding=move_request_object, **params)
    elif params.get('class_value') == 'result':
        result_object = Result(
            definition=params.get('definition'),
            scores=params.get('scores'),
            winner=params.get('winner'),
        )
        return data(class_binding=result_object, **params)
    elif params.get('class_value') == 'error':
        error_object = Error(
            message=params.get('message'),
            originalMessage=params.get('original_message'),
        )
        return data(class_binding=error_object, **params)
    else:
        logging.warn('Unknown class value: %s', params.get('class_value'))

    return data(**params)

def map_last_move(last_move: LastMove, params: dict):
    try:
        params.pop('class_binding')
    except KeyError:
        ...

    move_object = _socha.Move(
        start=_socha.Coordinate(x=params.get('from_').x, y=params.get('from_').y),
        direction=map_string_to_direction(params.get('direction')),
    )
    return last_move(class_binding=move_object, **params)


def custom_class_factory(clazz, params: dict):
    # print("TEST01: ", clazz, params)

    if clazz.__name__ == 'Data':
        return map_object(clazz, params)
    if clazz.__name__ == 'LastMove':
        test = map_last_move(clazz, params)
        return test

    return clazz(**params)


PROTOCOL_PREFIX = '<protocol>'.encode('utf-8')


class XMLProtocolInterface:
    """
    Serialize and deserialize objects to and from XML.
    """

    def __init__(self, host: str, port: int):
        self.network_interface = NetworkSocket(host, port)
        self.connect()
        self.running = False
        self.first_time = True

        context = XmlContext()
        deserialize_config = ParserConfig(class_factory=custom_class_factory)

        self.deserializer = XmlParser(
            handler=XmlEventHandler,
            context=context,
            config=deserialize_config,
        )

        serialize_config = SerializerConfig(
            pretty_print=True,
            xml_declaration=False,
            encoding='utf-8',
        )

        self.serializer = XmlSerializer(config=serialize_config)

    def connect(self):
        """
        Creates a TCP connection with the server.
        """
        self.network_interface.connect()

    def disconnect(self):
        """
        Sends a closing xml to the server and closes the connection afterward.
        """
        self.send(Close())
        self.network_interface.close()

    def _receive(self):
        """
        Gets a receiving byte stream from the server and deserializes it into an object.

        :return: The next object in the stream, or None if the server returns an empty response.
        """
        try:
            receiving = self.network_interface.receive()

            # Return None if the server returns an empty response
            if not receiving:
                return None
            
            # weird replacing of unicode chars, that are not working in xml rn
            unicodes = [b"\xe5", b"\xf6", b"\xfc", b"\xdf"]
            replaces = [b"ae", b"oe", b"ue", b"ss"]
            for i, t in enumerate(unicodes):
                receiving = receiving.replace(t, replaces[i])

            cls = self._deserialize_object(receiving)
            return cls
        except OSError:
            logging.error('An OSError occurred while receiving data from the server.')
            self.running = False
            raise
        except Exception as e:
            logging.error(
                'An error occurred while receiving data from the server: %s', e
            )
            self.running = False
            raise

    def send(self, obj: ProtocolPacket) -> None:
        """
        Sends an object to the server.

        :param obj: The object to send. Must not be `None`.
        """
        if obj is None:
            raise ValueError('Cannot send `None` to server')

        with self._encode_context() as encode:
            shipment = (
                PROTOCOL_PREFIX + encode(obj)
                if self.first_time is True
                else encode(obj)
            )

        try:
            self.network_interface.send(shipment)
        except Exception as e:
            logging.exception('Error sending shipment to server: %s', e)
            raise
        else:
            logging.debug('Sent shipment to server: %s', shipment)
        self.first_time = False

    @contextlib.contextmanager
    def _encode_context(self) -> Iterator[Callable[[Any], bytes]]:
        """
        A context manager that yields a function for encoding objects as bytes.
        """

        def encode(obj: Any) -> bytes:
            return self._serialize_object(obj)

        yield encode

    def _deserialize_object(self, byte_stream: bytes) -> ProtocolPacket:
        """
        Deserialize a xml byte stream to a ProtocolPacket.
        :param byte_stream: The byte stream to deserialize.
        :return: The deserialized ProtocolPacket child.
        """
        return self.deserializer.from_bytes(byte_stream)

    def _serialize_object(self, object_class: object) -> bytes:
        """
        Serialize a ProtocolPacket child to a xml byte stream.
        :param object_class: The ProtocolPacket child to serialize.
        :return: The serialized byte stream.
        """
        return self.serializer.render(object_class).encode('utf-8')
