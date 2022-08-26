import random

from src.socha.starter import Starter
from src.socha.api.networking.player_client import IClientHandler
from src.socha.api.plugin.penguins import GameState, Move


class Logic(IClientHandler):
    gameState: GameState

    def calculate_move(self) -> Move:
        mostFish = self.gameState.get_most_fish_moves()
        return mostFish[random.randint(0, len(mostFish) - 1)]

    def on_update(self, state: GameState):
        self.gameState = state

    def on_error(self, logMessage: str):
        ...


if __name__ == "__main__":
    Starter("Localhost", 13050, Logic())
