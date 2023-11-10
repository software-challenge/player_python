import logging
# not all imports are currently used, but they might be in the future and it shows all available functionalities
from socha import Accelerate, AccelerationProblem, Advance, AdvanceInfo, AdvanceProblem, Board
from socha import CartesianCoordinate, CubeCoordinates, CubeDirection, Field, FieldType, GameState
from socha import Move, Passenger, Push, PushProblem, Segment, Ship, TeamEnum, TeamPoints, Turn, TurnProblem
from socha.api.networking.game_client import IClientHandler
from socha.starter import Starter


class Logic(IClientHandler):
    game_state: GameState

    # this method is called every time the server is requesting a new move
    # this method should always be implemented otherwise the client will be disqualified
    def calculate_move(self) -> Move:
        logging.info("Calculate move...")
        return Move([Advance(1)])

    # this method is called every time the server has sent a new game state update
    # this method should be implemented to keep
    def on_update(self, state: GameState):
        self.game_state = state


if __name__ == "__main__":
    Starter(logic=Logic())
