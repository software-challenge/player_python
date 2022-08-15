from src.software_challenge_client.server_api.xtranslate.XTranslateInterface import *

protocol = protocolClasses


def alias(name: str):
    """
    Registers the alias name of a xString object representation.
    :param name: Name of the alias.
    :return:
    """

    def setName(obj):
        global protocol
        obj.__name__ = name
        protocol[name] = obj
        return obj

    return setName
