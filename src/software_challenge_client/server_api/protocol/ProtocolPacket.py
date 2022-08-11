"""
{
className : {
    type : classType,
    variableName1 : XMLDefinition,
    variableName2 : XMLDefinition
    }
}
"""
protocolClasses = {}


class ProtocolPacket:
    ...


class LobbyRequest(ProtocolPacket):
    ...


class AdminLobbyRequest(LobbyRequest):
    ...


class ResponsePacket(ProtocolPacket):
    ...
