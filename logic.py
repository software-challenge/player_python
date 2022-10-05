import random

from python.socha.socha import *
from python.socha.api.networking.player_client import IClientHandler
from python.socha.starter import Starter


class Logic(IClientHandler):
    gameState: GameState

    def calculate_move(self) -> Move:
        possible_moves = self.gameState.possible_moves(self.gameState.welcome_message.team)
        return possible_moves[random.randint(0, len(possible_moves) - 1)]

    def on_update(self, state: GameState):
        self.gameState = state


if __name__ == "__main__":
    Starter(logic=Logic())
