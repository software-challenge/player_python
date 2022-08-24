from typing import Any

from src.SoftwareChallengeClient.api.Shared import protocolClasses, attributeReference


def alias(name: str):
    """
    Registers the alias name of a xString object representation.
    :param name: Name of the alias.
    :return:
    """

    def setName(obj):
        protocolClasses[name] = obj
        protocolClasses[obj] = name
        return obj

    return setName


def childAttribute(name: str, mappedClass: Any):
    """

    :param name:
    :param mappedClass:
    :return:
    """

    def setChildAttribute(obj):
        protocolClasses[name] = mappedClass
        return obj

    return setChildAttribute


def attrDict(attr: str, name: str):
    """

    :param attr:
    :param name:
    :return:
    """

    def setAttrDict(obj):
        attributeReference[name] = attr
        attributeReference[attr] = name
        return obj

    return setAttrDict


def setClassName(name: str):
    """
    Registers the alias name of a xString object representation.
    :param name: Name of the alias.
    :return:
    """

    def setName(obj):
        obj.__name__ = name
        return obj

    return setName
