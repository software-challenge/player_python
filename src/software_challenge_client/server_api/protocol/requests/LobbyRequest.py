import src.software_challenge_client.server_api.decorations as xml
import paxb as pb

from dataclasses import dataclass

from src.software_challenge_client.server_api.protocol.ProtocolPacket import LobbyRequest


@dataclass
@xml.model(name='join')
class JoinGameRequest(LobbyRequest):
    gameType: str = xml.asAttribute()


@dataclass
@xml.model(name='joinPrepared')
class JoinPreparedRoomRequest(LobbyRequest):
    reservationCode: str = None


@dataclass
@xml.model(name='joinRoom')
class JoinRoomRequest(LobbyRequest):
    roomId: str = None
