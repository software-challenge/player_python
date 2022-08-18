from src.SoftwareChallengeClient.player.PlayerClient import IGameHandler
from src.SoftwareChallengeClient.server_api.networking.clients.LobbyClient import LobbyClient
from src.SoftwareChallengeClient.server_api.protocol.Shared import GameResult
from src.SoftwareChallengeClient.server_api.protocol.room.IRoomMessage import RoomMessage
from src.SoftwareChallengeClient.server_api.sc.api.plugins.IPlugins import IGameState


class ILogic(IGameHandler):
    def calculateMove(self) -> RoomMessage: ...

    def onUpdate(self, state: IGameState): ...

    def onGameOver(self, roomMessage: GameResult): ...

    def onError(self, logMessage: str): ...


class Starter:

    def __init__(self, host: str, port: int, reservation: str = None, roomId: str = None,
                 logic: IGameHandler = ILogic(), keepAlive: bool = False):
        self.host = host
        self.port = port
        self.reservation = reservation
        self.roomId = roomId
        self.logic = logic
        self.client = LobbyClient(host, port).asPlayer(self.logic, keepAlive)

        if reservation:
            self.client.joinGameWithReservation(reservation)
        elif roomId:
            self.client.joinGameRoom(roomId)
        else:
            self.client.joinGame()

        self.client.start()
