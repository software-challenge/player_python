import random

from socha.api.networking.player_client import IClientHandler
from socha.api.plugin.penguins import *
from socha.starter import Starter


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
