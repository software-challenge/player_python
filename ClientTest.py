from src.SoftwareChallengeClient.api.networking.clients.PlayerClient import IClientHandler
from src.SoftwareChallengeClient.api.sc.Plugin2023 import GameState, Move
from src.SoftwareChallengeClient.player.Starter import Starter


class Logic(IClientHandler):
    gameState: GameState

    def calculateMove(self) -> Move:
        return self.gameState.getPossibleMoves()[0]

    def onUpdate(self, state: GameState):
        self.gameState = state


if __name__ == "__main__":
    Starter("Localhost", 13050, Logic())
