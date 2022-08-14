from dataclasses import dataclass

import src.software_challenge_client.server_api.XStreamDecorator as XStrDec
from src.software_challenge_client.server_api.Shared import SlotDescriptor
from src.software_challenge_client.server_api.protocol.ProtocolPacket import AdminLobbyRequest


@dataclass
@XStrDec.alias(name='authenticate')
class AuthenticateRequest(AdminLobbyRequest):
    password: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name='cancel')
class CancelRequest(AdminLobbyRequest):
    roomId: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name='observe')
class ObservationRequest(AdminLobbyRequest):
    roomId: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name='pause')
class PauseGameRequest(AdminLobbyRequest):
    roomId: str = XStrDec.asAttribute()


@dataclass
@XStrDec.alias(name='prepare')
class PrepareGameRequest(AdminLobbyRequest):
    gameType: str = XStrDec.asAttribute()
    slotDescriptors: list[SlotDescriptor] = XStrDec.implicitArray(itemFieldName="slot",
                                                                  default=[SlotDescriptor(displayName="Player1"),
                                                                           SlotDescriptor(displayName="Player1")])
    pause: bool = XStrDec.asAttribute()

    def __eq__(self, other) -> bool:
        return self.gameType == other.gameType and self.slotDescriptors == other.slotDescriptors and \
               self.pause == other.pause

    def __hash__(self) -> int:
        return (hash(self.gameType) * 31 + hash(self.slotDescriptors)) * 31 + hash(self.pause)


@dataclass
@XStrDec.alias(name='step')
class StepRequest(AdminLobbyRequest):
    roomId: str = XStrDec.asAttribute()
    forced: bool = XStrDec.asAttribute(default=False)
