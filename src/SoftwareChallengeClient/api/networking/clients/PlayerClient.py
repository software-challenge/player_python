from src.SoftwareChallengeClient.api.networking.xflux.XFlux import XFluxClient
from src.SoftwareChallengeClient.api.protocol import JoinGameRequest, JoinRoomRequest, \
    JoinPreparedRoomRequest, GamePreparedResponse, JoinedRoomResponse, ObservationResponse, ErrorPacket, \
    RoomWasJoinedEvent
from src.SoftwareChallengeClient.api.protocol.RoomPacket import RoomPacket
from src.SoftwareChallengeClient.api.protocol.SharedProtocols import RemovedFromGame, GameResult
from src.SoftwareChallengeClient.api.protocol.room.ErrorMessage import ErrorMessage
from src.SoftwareChallengeClient.api.protocol.room.IRoomMessage import RoomMessage, ObservableRoomMessage
from src.SoftwareChallengeClient.api.protocol.room.RoomMessage import MementoMessage, MoveRequest
from src.SoftwareChallengeClient.api.sc.Plugin2023 import GameState, Move


class IClientHandler:
    def calculateMove(self) -> Move: ...

    def onUpdate(self, state: GameState): ...

    def onGameOver(self, roomMessage: GameResult): ...

    def onError(self, logMessage: str): ...

    def onRoomMessage(self, data): ...

    def onGamePrepared(self, message): ...

    def onGameJoined(self, message): ...

    def onGameObserved(self, message): ...


class PlayerClient(XFluxClient):
    def __init__(self, host: str, port: int, handler: IClientHandler, keepAlive: bool):
        super().__init__(host, port)
        self.gameHandler = handler
        self.keepAlive = keepAlive

    def authenticate(self, password: str, consumer):
        ...

    def observeRoom(self, roomId: str, observer):
        ...

    def joinGame(self, gameType: str = None):
        super().send(JoinGameRequest(gameType))

    def joinGameRoom(self, roomId: str):
        super().send(JoinRoomRequest(roomId))

    def joinGameWithReservation(self, reservation: str):
        super().send(JoinPreparedRoomRequest(reservation))

    def sendMessageToRoom(self, roomId: str, message):
        # print(roomId, message)
        super().send(RoomPacket(roomId, message))

    def onObject(self, message):
        if isinstance(message, RoomPacket):
            roomId: str = message.getRoomId()
            data: RoomMessage = message.getData()
            if isinstance(data, MoveRequest):
                response = self.gameHandler.calculateMove()
                if response:
                    # TODO Logger
                    self.sendMessageToRoom(roomId, response)
            if isinstance(data, ObservableRoomMessage):
                # TODO Set observer data
                if isinstance(data, MementoMessage):
                    self.gameHandler.onUpdate(data.state)
                elif isinstance(data, GameResult):
                    self.gameHandler.onGameOver(data)
                elif isinstance(data, ErrorMessage):
                    # TODO Logger
                    self.gameHandler.onError(str(message))
            else:
                self.gameHandler.onRoomMessage(data)
        elif isinstance(message, RemovedFromGame):
            roomId: str = message.roomId
            # TODO Logger
            if not self.keepAlive:
                super().stop()
        elif isinstance(message, GamePreparedResponse):
            self.gameHandler.onGamePrepared(message)
        elif isinstance(message, JoinedRoomResponse):
            self.gameHandler.onGameJoined(message)
        elif isinstance(message, RoomWasJoinedEvent):
            self.gameHandler.onGameJoined(message)
        elif isinstance(message, ObservationResponse):
            self.gameHandler.onGameObserved(message)
        elif isinstance(message, ErrorPacket):
            self.gameHandler.onError(str(message))
