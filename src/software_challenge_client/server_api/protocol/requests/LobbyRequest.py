from dataclasses import dataclass

import src.software_challenge_client.server_api.XStreamDecorator as XStrDec
from src.software_challenge_client.server_api.protocol.ProtocolPacket import LobbyRequest


@dataclass
@XStrDec.alias(name='join')
class JoinGameRequest(LobbyRequest):
    gameType: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name='joinPrepared')
class JoinPreparedRoomRequest(LobbyRequest):
    reservationCode: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name='joinRoom')
class JoinRoomRequest(LobbyRequest):
    roomId: str = XStrDec.asAttribute()
