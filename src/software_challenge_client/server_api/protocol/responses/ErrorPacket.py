from dataclasses import dataclass

import src.software_challenge_client.server_api.XStreamDecorator as XStrDec
from src.software_challenge_client.server_api.protocol.ProtocolPacket import ResponsePacket, ProtocolPacket


@dataclass
@XStrDec.alias(name="errorpacket")
class ErrorPacket(ResponsePacket):
    originalRequest: ProtocolPacket = None
    message: str = XStrDec.asAttribute()
