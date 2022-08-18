from src.SoftwareChallengeClient.player.PlayerClient import IGameHandler, IPlayerClient, PlayerClient
from src.SoftwareChallengeClient.server_api.networking.xflux.XFlux import XFluxClient
from src.SoftwareChallengeClient.server_api.protocol import JoinGameRequest, JoinRoomRequest, \
    JoinPreparedRoomRequest, GamePreparedResponse, JoinedRoomResponse, ObservationResponse, ErrorPacket, \
    RoomWasJoinedEvent
from src.SoftwareChallengeClient.server_api.protocol.RoomPacket import RoomPacket
from src.SoftwareChallengeClient.server_api.protocol.Shared import RemovedFromGame, GameResult
from src.SoftwareChallengeClient.server_api.protocol.room.ErrorMessage import ErrorMessage
from src.SoftwareChallengeClient.server_api.protocol.room.IRoomMessage import RoomMessage, ObservableRoomMessage
from src.SoftwareChallengeClient.server_api.protocol.room.RoomMessage import MementoMessage


class ILobbyClientListener:
    def onError(self, error): ...

    def onNewState(self, roomId, state): ...

    def onRoomMessage(self, roomId, data): ...

    def onGamePrepared(self, response): ...

    def onGameJoined(self, roomId): ...

    def onGameObserved(self, roomId): ...


class IHistoryListener:
    def onNewState(self, roomId, message): ...


class LobbyClient(XFluxClient):
    player: IPlayerClient
    keepAlive: bool
    listeners: list[ILobbyClientListener]
    historyListeners: list[IHistoryListener]

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
                    # Logger
                    self.sendMessageToRoom(roomId, response)
            if message is ObservableRoomMessage:
                # Set observer data
                if message is MementoMessage:
                    self.__onNewState(roomId, message.state)
                elif message is GameResult:
                    self.__onGameOver(roomId, message)
                elif message is ErrorMessage:
                    # Logger
                    error: ErrorPacket = message
                    for listener in self.listeners:
                        listener.onError(error)
            else:
                self.__onRoomMessage(roomId, data)
        elif message is RemovedFromGame:
            roomId: str = message.roomId
            # Logger
            if not self.keepAlive:
                super().stop()
        elif message is GamePreparedResponse:
            self.__onGamePrepared(message)
        elif message is JoinedRoomResponse:
            self.__onGameJoined(message.getRoomId())
        elif message is RoomWasJoinedEvent:
            self.__onGameJoined(message.getRoomId())
        elif message is ObservationResponse:
            self.__onGameObserved(message.getRoomId())
        elif message is ErrorPacket:
            error: ErrorPacket = message
            for listener in self.listeners:
                listener.onError(error)
        else:
            self.__onCustomObject()

    def __onNewState(self, roomId, state):
        for listener in self.listeners:
            listener.onNewState(roomId, state)

    def __onGameOver(self, roomId, message):
        # Logger
        for listener, historyListeners in self.listeners, self.historyListeners:
            listener.onGameOver(roomId, message)
            historyListeners.onGameOver(roomId, message)

    def __onRoomMessage(self, roomId, data):
        for listener in self.listeners:
            listener.onRoomMessage(roomId, data)

    def __onGamePrepared(self, response):
        for listener in self.listeners:
            listener.onGamePrepared(response)

    def __onGameJoined(self, roomId):
        for listener in self.listeners:
            listener.onGameJoined(roomId)

    def __onGameObserved(self, roomId):
        for listener in self.listeners:
            listener.onGameObserved(roomId)

    def __onCustomObject(self):
        # Logger
        ...
