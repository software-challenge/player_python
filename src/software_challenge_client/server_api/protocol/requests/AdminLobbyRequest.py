import src.software_challenge_client.server_api.decorations as xml

from dataclasses import dataclass

from src.software_challenge_client.server_api.protocol.ProtocolPacket import AdminLobbyRequest


@dataclass
@xml.model(name='authenticate')
class AuthenticateRequest(AdminLobbyRequest):
    password: str = None


@dataclass
@xml.model(name='cancel')
class CancelRequest(AdminLobbyRequest):
    roomId: str = None


@dataclass
@xml.model(name='observe')
class ObservationRequest(AdminLobbyRequest):
    roomId: str = None


@dataclass
@xml.model(name='pause')
class PauseGameRequest(AdminLobbyRequest):
    roomId: str = None


@dataclass
@xml.model(name='prepare')
class PrepareGameRequest(AdminLobbyRequest):
    gameType: str = None
    slotDescriptors: str = None
    pause: bool = None


@dataclass
@xml.model(name='step')
class StepRequest(AdminLobbyRequest):
    roomId: str = None
    forced: bool = None
