from xsdata.formats.dataclass.context import XmlContext
from xsdata.formats.dataclass.parsers import XmlParser
from xsdata.formats.dataclass.parsers.config import ParserConfig
from xsdata.formats.dataclass.parsers.handlers import XmlEventHandler
from xsdata.formats.dataclass.serializers import XmlSerializer
from xsdata.formats.dataclass.serializers.config import SerializerConfig

from src.SoftwareChallengeClient.api.protocol.protocol import *
from src.SoftwareChallengeClient.api.protocol.room.RoomMessage import MoveRequest
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


def deserializeObject(byteStream: bytes):
    context = XmlContext()
    config = ParserConfig(class_factory=customClassFactory)
    parser = XmlParser(handler=XmlEventHandler, context=context, config=config)
    parsed = parser.from_bytes(byteStream)
    print(parsed)
    return parsed


def serializeObject(objectClass: object):
    config = SerializerConfig(pretty_print=True)
    serializer = XmlSerializer(config=config)

    if isinstance(objectClass, Move):
        from_value = From(x=objectClass.from_value.x, y=objectClass.from_value.y)
        to = To(x=objectClass.to.x, y=objectClass.to.y)
        data = Data(class_value="move", from_value=from_value, to=to)
        return serializer.render(data)

    return serializer.render(objectClass)
