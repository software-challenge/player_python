from logging import DEBUG
from socha import GameState, Move, Starter, Advance, Turn, CubeDirection
from socha.api.networking.game_client import IClientHandler


class Logic(IClientHandler):
    game_state: GameState

    def calculate_move(self) -> Move:
        return Move(actions=[Advance(1)])

    def on_update(self, state: GameState):
        self.game_state = state


if __name__ == "__main__":
    Starter(logic=Logic())
