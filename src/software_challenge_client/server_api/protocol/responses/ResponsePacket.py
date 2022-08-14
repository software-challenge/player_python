from dataclasses import dataclass

import src.software_challenge_client.server_api.XStreamDecorator as XStrDec
from src.software_challenge_client.server_api.protocol.ProtocolPacket import ResponsePacket


@dataclass
@XStrDec.alias(name="prepared")
class GamePreparedResponse(ResponsePacket):
    roomId: str = XStrDec.asAttribute()
    reservations: list[str] = XStrDec.implicitArray(itemFieldName="reservation")


@dataclass
@XStrDec.alias(name="joined")
class JoinedRoomResponse(ResponsePacket):
    roomId: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name="observed")
class ObservationResponse(ResponsePacket):
    roomId: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name="joinedGameRoom")
class RoomJoinedResponse(ResponsePacket):
    """
    Originally named **RoomWasJoinedEvent**
    """
    roomId: str = XStrDec.asAttribute()
    playerCount: int = XStrDec.asAttribute()
