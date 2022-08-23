import logging
import random
import time

from src.SoftwareChallengeClient.api.networking.clients.PlayerClient import IClientHandler
from src.SoftwareChallengeClient.api.sc.Plugin2023 import GameState, Move
from src.SoftwareChallengeClient.player.Starter import Starter


class Logic(IClientHandler):
    gameState: GameState

    def calculateMove(self) -> Move:
        startTime = time.time()
        possibleMoves = self.gameState.getPossibleMoves()
        move = possibleMoves[random.randint(0, len(possibleMoves) - 1)]
        logging.info("Sent {} after {} seconds.".format(move, time.time() - startTime))
        return move

    def onUpdate(self, state: GameState):
        self.gameState = state

    def onError(self, logMessage: str):
        ...


if __name__ == "__main__":
    Starter("Localhost", 13050, Logic())
