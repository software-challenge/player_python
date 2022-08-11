import inspect
import sys

from src.software_challenge_client.server_api.protocol import *

"""
{
className : {
    type : classType,
    variableName1 : XMLDefinition,
    variableName2 : XMLDefinition
    }
}
"""
protocol = protocolClasses


def model(name: str):
    """
    Registers the model name of a xString object representation.
    :param name: Name of the model.
    :return:
    """

    def setName(obj):
        global protocol
        protocol[name] = obj().__class__
        return obj

    return setName


def asAttribute():
    """
    Registers the field as an xml attribute.
    """
    global protocol
    callerClass = inspect.stack()[1][3]
    fields = inspect.stack()[1][4]
    protocol[callerClass] = {}
    classDict = protocol.get(callerClass)
    for field in fields:
        fieldName = field.split(":")[0].strip()
        fieldXmlType = field.split(".")[1].split("()")[0].strip()

        classDict[fieldName] = fieldXmlType
