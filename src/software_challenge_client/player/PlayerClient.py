from src.software_challenge_client.server_api.networking.xflux.XFlux import XFluxClient
from src.software_challenge_client.server_api.protocol.Shared import GameResult
from src.software_challenge_client.server_api.protocol.room.ErrorMessage import ErrorMessage
from src.software_challenge_client.server_api.protocol.room.IRoomMessage import RoomMessage
from src.software_challenge_client.server_api.protocol.room.RoomMessage import MoveRequest, MementoMessage
from src.software_challenge_client.server_api.sc.api.plugins.IPlugins import IGameState


class IGameHandler:
    def calculateMove(self) -> RoomMessage: ...

    def onUpdate(self, state: IGameState): ...

    def onGameOver(self, roomMessage: GameResult): ...

    def onError(self, logMessage: str): ...


class IPlayerClient:

    def joinGameWithReservation(self, reservation: str):
        ...

    def joinGameRoom(self, roomId: str):
        ...

    def joinGame(self, gameType: str = None):
        ...

    def processMessage(self, roomMessage) -> RoomMessage: ...


class PlayerClient(IPlayerClient):

    def __init__(self, client: XFluxClient, handler: IGameHandler):
        self.client = client
        self.handler = handler

    def processMessage(self, roomMessage) -> RoomMessage:
        if roomMessage is MoveRequest:
            return self.handler.calculateMove()
        elif roomMessage is MementoMessage:
            self.handler.onUpdate(roomMessage.state)
        elif roomMessage is GameResult:
            self.handler.onGameOver(roomMessage)
        elif roomMessage is ErrorMessage:
            self.handler.onError(roomMessage.getLogMessage())
