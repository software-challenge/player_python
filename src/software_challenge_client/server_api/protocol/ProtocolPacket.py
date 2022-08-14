"""
{
    className : {
        variableName : {
            "type" : XmlType,
            "itemFieldName": itemFieldName
        }
    },
    XmlAlias: class
}
"""
protocolClasses: dict = {}


class ProtocolPacket:
    ...


class LobbyRequest(ProtocolPacket):
    ...


class AdminLobbyRequest(LobbyRequest):
    ...


class ResponsePacket(ProtocolPacket):
    ...
