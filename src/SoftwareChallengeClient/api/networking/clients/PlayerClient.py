from src.SoftwareChallengeClient.api.networking.xflux.XFlux import XFluxClient

from src.SoftwareChallengeClient.api.protocol.Protocol import *
from src.SoftwareChallengeClient.api.sc import Plugin2023
from src.SoftwareChallengeClient.api.sc.Plugin2023 import Field, GameState, Move, HexBoard


def convertBoard(protocolBoard: Board) -> Plugin2023.Board:
    boardList: list[list[Field]] = []
    for row in protocolBoard.list_value:
        rowList: list[Field] = []
        for fields in row.field_value:
            rowList.append(Field(fields))
        boardList.append(rowList)
    return Plugin2023.Board(HexBoard([list(x) for x in zip(*boardList)]))


class IClientHandler:
    def calculateMove(self) -> Move: ...

    def onUpdate(self, state: GameState): ...

    def onGameOver(self, roomMessage: Result): ...

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
        super().send(Join())

    def joinGameRoom(self, roomId: str):
        super().send(JoinRoom(room_id=roomId))

    def joinGameWithReservation(self, reservation: str):
        super().send(JoinPrepared(reservation_code=reservation))

    def sendMessageToRoom(self, roomId: str, message):
        super().send(Room(room_id=roomId, data=message))

    def onObject(self, message):
        if isinstance(message, Room):
            roomId: str = message.room_id
            data: RoomMessage = message.data.class_binding
            if isinstance(data, MoveRequest):
                response = self.gameHandler.calculateMove()
                if response:
                    from_value = None
                    to = To(x=response.to.x, y=response.to.y)
                    if response.from_value:
                        from_value = From(x=response.from_value.x, y=response.from_value.y)
                    response = Data(class_value="move", from_value=from_value, to=to)
                    # TODO Logger
                    self.sendMessageToRoom(roomId, response)
            if isinstance(data, ObservableRoomMessage):
                # TODO Set observer data
                if isinstance(data, State):
                    gameState = GameState(turn=data.turn, startTeam=Team(data.start_team),
                                          board=convertBoard(data.board), lastMove=data.last_move,
                                          fishes=Plugin2023.Fishes(data.fishes.int_value[0], data.fishes.int_value[1]))
                    self.gameHandler.onUpdate(gameState)
                elif isinstance(data, Result):
                    self.gameHandler.onGameOver(data)
                elif isinstance(data, object):
                    # TODO Logger
                    self.gameHandler.onError(str(message))
            else:
                self.gameHandler.onRoomMessage(data)
        elif isinstance(message, JoinedGameRoom):
            self.gameHandler.onGameJoined(message)
        elif isinstance(message, object):
            self.gameHandler.onError(str(message))
