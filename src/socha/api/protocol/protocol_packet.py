class ProtocolPacket:
    ...


class LobbyRequest(ProtocolPacket):
    ...


class AdminLobbyRequest(LobbyRequest):
    ...


class ResponsePacket(ProtocolPacket):
    ...
