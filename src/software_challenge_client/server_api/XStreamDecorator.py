import inspect
from dataclasses import field

from src.software_challenge_client.server_api.protocol import *

protocol = protocolClasses


def alias(name: str):
    """
    Registers the alias name of a xString object representation.
    :param name: Name of the alias.
    :return:
    """

    def setName(obj):
        global protocol
        protocol[name] = obj().__class__
        return obj

    return setName


def _dictEntry(stack):
    callerClass = stack[1][3]
    callerField = stack[1][4][0]

    if callerClass not in protocol:
        protocol[callerClass] = {}
    classDict = protocol.get(callerClass)

    fieldName = callerField.split(":")[0].strip()
    fieldXmlType = callerField.split(".")[1].split("(")[0].strip()

    return classDict, fieldName, fieldXmlType


def asAttribute(default=None):
    """
    Registers the field as an XStrDec attribute.
    """
    global protocol

    stack = inspect.stack()
    classDict, fieldName, fieldXmlType = _dictEntry(stack)

    classDict[fieldName] = {"type": fieldXmlType}

    return default


def implicitArray(itemFieldName, default=None):
    global protocol

    stack = inspect.stack()
    classDict, fieldName, fieldXmlType = _dictEntry(stack)

    classDict[fieldName] = {"type": fieldXmlType,
                            "itemFieldName": itemFieldName}

    return field(default_factory=lambda: default)
