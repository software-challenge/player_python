from src.SoftwareChallengeClient.server_api.protocol import ProtocolPacket


class IClient:
    def send(self, packet: ProtocolPacket):
        ...

    def receive(self) -> ProtocolPacket:
        ...
