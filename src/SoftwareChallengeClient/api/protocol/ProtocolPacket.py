"""
{
    XmlAlias: class,
    ...
}
"""
protocolClasses: dict = {}
attributeReference: dict = {}


class ProtocolPacket:
    ...


class LobbyRequest(ProtocolPacket):
    ...


class AdminLobbyRequest(LobbyRequest):
    ...


class ResponsePacket(ProtocolPacket):
    ...
