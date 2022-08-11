from dataclasses import dataclass

import src.software_challenge_client.server_api.decorations as xml
from src.software_challenge_client.server_api.protocol.ProtocolPacket import ResponsePacket, ProtocolPacket


@dataclass
@xml.model(name="errorpacket")
class ErrorPacket(ResponsePacket):
    originalRequest: ProtocolPacket = None
    message: str = None
