import random

from src.SoftwareChallengeClient.api.networking.clients.PlayerClient import IClientHandler
from src.SoftwareChallengeClient.api.sc.Plugin2023 import GameState, Move
from src.SoftwareChallengeClient.player.Starter import Starter


class Logic(IClientHandler):
    gameState: GameState

    def calculateMove(self) -> Move:
        print(self.gameState.currentTeam)
        possibleMoves = self.gameState.getPossibleMoves()
        return possibleMoves[random.randint(0, len(possibleMoves) - 1)]

    def onUpdate(self, state: GameState):
        self.gameState = state

    def onError(self, logMessage: str):
        ...


if __name__ == "__main__":
    Starter("Localhost", 13050, Logic())
