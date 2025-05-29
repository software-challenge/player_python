# not all imports are currently used, but they might be in the future and it shows all available functionalities
import math
import random
import time
from typing import Optional, Tuple
from socha import *
from socha.api.networking.game_client import IClientHandler
from socha.starter import Starter



class Logic(IClientHandler):
    game_state: GameState

    # this method is called every time the server is requesting a new move
    # this method should always be implemented otherwise the client will be disqualified
    def calculate_move(self) -> Move:

        scores = []

        i = 0
        for p in self.poss:
            scores.append(0)

            '''
            if isinstance(p.action, Advance):
                scores[-1] += i*2
                dest = p.action.distance + self.game_state.clone_current_player().position

                if self.game_state.board.get_field(dest) == Field.Market:
                    if Card.HurryAhead in p.action.cards and len(self.game_state.clone_current_player().cards) < 2:
                        scores[-1] += 1000

                        
            '''
            if isinstance(p.action, Advance):
                scores[-1] += i*2

                dest = p.action.distance + self.game_state.clone_current_player().position
                if dest == 63:
                    scores[-1] += 1000
            
            if isinstance(p.action, FallBack):
                if self.game_state.clone_current_player().carrots <= 10 and self.game_state.clone_current_player().position != 63:
                    scores[-1] += 10000

            if isinstance(p.action, ExchangeCarrots):
                if self.game_state.clone_current_player().position == 63:
                    scores[-1] += (1000 * 999)

            i += 1

        index = 0
        highest = -1

        i = 0
        for s in scores:
            if s >= highest:
                highest = s
                index = i

            i += 1

        print(scores, highest, index)
        return self.poss[index]

    # this method is called every time the server has sent a new game state update
    # this method should be implemented to keep the game state up to date
    def on_update(self, state: GameState) -> None:
        self.game_state = state

        #self.poss_old = self.game_state.possible_moves_old()
        self.poss = self.game_state.possible_moves()

        print("")
        print("turn:", self.game_state.turn)
        print("current:", self.game_state.clone_current_player().team)
        print("moves old:")
        #for p in self.poss_old:
            #print(p)
        print("moves:")
        for p in self.poss:
            print(p)


if __name__ == "__main__":
    Starter(logic=Logic())
    # if u wanna have more insights, u can set the logging level to debug:
    # Starter(logic=Logic(), log_level=logging.DEBUG)
