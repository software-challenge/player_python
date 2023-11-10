# not all imports are currently used, but they might be in the future and it shows all available functionalities
from socha import Accelerate, AccelerationProblem, Advance, AdvanceInfo, AdvanceProblem, Board
from socha import CartesianCoordinate, CubeCoordinates, CubeDirection, Field, FieldType, GameState
from socha import Move, Passenger, Push, PushProblem, Segment, Ship, TeamEnum, TeamPoints, Turn, TurnProblem
from socha.api.networking.game_client import IClientHandler
from socha.starter import Starter


class Logic(IClientHandler):
    game_state: GameState

    def calculate_move(self) -> Move:
        return Move([Advance(1)])

    def on_update(self, state: GameState):
        self.game_state = state


if __name__ == "__main__":
    Starter(logic=Logic())
