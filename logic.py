import random

from socha import *


class Logic(IClientHandler):
    game_state: GameState

    def calculate_move(self) -> Move:
        possible_moves = self.game_state.possible_moves(self.game_state.current_team().name)
        return possible_moves[random.randint(0, len(possible_moves) - 1)]

    def on_update(self, state: GameState):
        self.game_state = state


if __name__ == "__main__":
    Starter(logic=Logic())
