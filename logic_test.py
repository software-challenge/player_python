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

        poss = self.game_state.possible_moves_old()

        poss_performance = self.game_state.possible_moves()

        print("poss", len(poss))
        print("performance", len(poss_performance))
        print("same:", poss == poss_performance)
        print("same items:", self.compare_move_lists(poss, poss_performance))

        card_moves = []
        for p in poss:
            print(p)
            if isinstance(p.action, Advance):
                if len(p.action.cards) > 0:
                    card_moves.append(p)

        if len(card_moves) > 0:
            return random.choice(card_moves)

        return random.choice(poss)

    # this method is called every time the server has sent a new game state update
    # this method should be implemented to keep the game state up to date
    def on_update(self, state: GameState) -> None:
        self.game_state = state


    def compare_move_lists(self, one, two) -> bool:

        if len(one) != len(two):
            return False
        
        if len(one) == 0 and len(two) == 0:
            return True
        
        found = 0

        for o in one:
            if o in two:
                found += 1

        if found == len(two):
            return True
        else:
            return False



if __name__ == "__main__":
    Starter(logic=Logic())
    # if u wanna have more insights, u can set the logging level to debug:
    # Starter(logic=Logic(), log_level=logging.DEBUG)
