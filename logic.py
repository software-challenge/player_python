import logging
import random

from socha.socha import *
from socha.api.networking.player_client import IClientHandler
from socha.starter import Starter


class Logic(IClientHandler):
    game_state: GameState

    def calculate_move(self) -> Move:
        logging.info(self.game_state.board)
        possible_moves = self.game_state.possible_moves(self.game_state.welcome_message.team)
        return possible_moves[random.randint(0, len(possible_moves) - 1)]

    def on_update(self, state: GameState):
        self.game_state = state


if __name__ == "__main__":
    Starter(logic=Logic())
