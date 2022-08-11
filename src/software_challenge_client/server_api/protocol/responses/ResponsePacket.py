from dataclasses import dataclass

import src.software_challenge_client.server_api.decorations as xml
from src.software_challenge_client.server_api.protocol.ProtocolPacket import ResponsePacket


@dataclass
@xml.model(name="prepared")
class GamePreparedResponse(ResponsePacket):
    roomId: str = None
    reservations: list = None


@dataclass
@xml.model(name="joined")
class JoinedRoomResponse(ResponsePacket):
    roomId: str = None


@dataclass
@xml.model(name="observed")
class ObservationResponse(ResponsePacket):
    roomId: str = None


@dataclass
@xml.model(name="joinedGameRoom")
class RoomJoinedResponse(ResponsePacket):
    roomId: str = None
    playerCount: int = None
