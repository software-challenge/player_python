from src.software_challenge_client.player.PlayerClient import IGameHandler, IPlayerClient, PlayerClient
from src.software_challenge_client.server_api.networking.xflux.XFlux import XFluxClient
from src.software_challenge_client.server_api.protocol import JoinGameRequest, JoinRoomRequest, \
    JoinPreparedRoomRequest
from src.software_challenge_client.server_api.protocol.RoomPacket import RoomPacket
from src.software_challenge_client.server_api.protocol.room.IRoomMessage import RoomMessage


class LobbyClient(XFluxClient):
    player: IPlayerClient
    keepAlive: bool

    def __init__(self, host: str, port: int):
        super().__init__(host, port)

    def authenticate(self, password: str, consumer):
        ...

    def observeRoom(self, roomId: str, observer):
        ...

    def asPlayer(self, handler: IGameHandler, keepAlive: bool) -> IPlayerClient:
        client: PlayerClient = PlayerClient(self, handler)
        self.player = client
        self.keepAlive = keepAlive
        return client

    def joinGame(self, gameType: str = None):
        super().send(JoinGameRequest(gameType))

    def joinGameRoom(self, roomId: str):
        super().send(JoinRoomRequest(roomId))

    def joinGameWithReservation(self, reservation: str):
        super().send(JoinPreparedRoomRequest(reservation))

    def sendMessageToRoom(self, roomId: str, message: RoomMessage):
        super().send(RoomPacket(roomId, message))

    def onObject(self, message):
        if message is RoomPacket:
            roomId: str = message.getRoomId()
            data: RoomMessage = message.getData()
            if self.player:
                response: RoomMessage = self.player.processMessage(data)
                if response:
                    self.sendMessageToRoom(roomId, response)
