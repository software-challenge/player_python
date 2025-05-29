# not all imports are currently used, but they might be in the future and it shows all available functionalities
import random
from socha import (
    Field,
    GameState,
    Move,
)
from socha.api.networking.game_client import IClientHandler
from socha.starter import Starter
import random
import time


class Logic(IClientHandler):
    game_state: GameState
    
    totalTimesOldSum = 0
    totalTimesPerformanceSum = 0
    count = 0
    countSame = 0
    countSameItems = 0
    
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

    
    def on_game_over(self, roomMessage):
        print("avg old:", self.totalTimesOldSum / self.count)
        print("avg perf:", self.totalTimesPerformanceSum / self.count)
        print("avg multi:", (self.totalTimesOldSum / self.count) / (self.totalTimesPerformanceSum / self.count))
        print("avg same:", self.countSame / self.count)
        print("avg same items:", self.countSameItems / self.count)
    
    # this method is called every time the server is requesting a new move
    # this method should always be implemented otherwise the client will be disqualified
    def calculate_move(self) -> Move:
        start_time = time.time()
        our_player = self.game_state.clone_current_player()

        start_3 = time.time()

        possible_moves = self.game_state.possible_moves_old()

        totalTime3 = time.time() - start_3
        print("3", totalTime3, len(possible_moves))
        self.totalTimesOldSum += totalTime3

        start_4 = time.time()

        possible_moves_performance = self.game_state.possible_moves()

        totalTime4 = (time.time() - start_4) + 0.000001
        print("4", totalTime4, len(possible_moves_performance))
        self.totalTimesPerformanceSum += totalTime4

        print("time multi:", totalTime3 / totalTime4)

        start_time_2 = time.time()
        board = [] 
        shop_moves = []
        hedgehog_moves = []

        print("poss", len(possible_moves))
        print("performance", len(possible_moves_performance))
        print("same:", possible_moves == possible_moves_performance)
        print("same items:", self.compare_move_lists(possible_moves, possible_moves_performance))

        print("moves:")
        for p in possible_moves_performance:
            print(p)

        self.count += 1
        if possible_moves == possible_moves_performance:
            self.countSame += 1
        if self.compare_move_lists(possible_moves, possible_moves_performance):
            self.countSameItems += 1

        # Einmal alle Felder in als Liste board, damit ich leichter drüber iterieren kann.
        def fields_on_board(board):
            for i in range (0, 64):
                board.append(self.game_state.board.get_field(i))
    
        
        fields_on_board(board)
        
        

        #Simuliert einmal alle mögliche Züge, und zwischenspeichert diese in new_state, um dann zu prüfen, ob die neue Position auf einem Shopfeld ist. Wenn ja, wird der rohe Move in die Shop_moves Liste hinzugefügt.
        def move_for_nearest_important_fields(possible_moves, board, shop_moves, hedgehog_moves, our_player):
            
            evantual_hedgehog_move = possible_moves[-1]
            
            possible_moves = possible_moves [:7]
            # print (possible_moves)
            possible_moves.append(evantual_hedgehog_move)
            
            try:
                for i in range (0, len(possible_moves)): 
                    new_state = self.game_state.perform_move(possible_moves[i])
                    our_player_new_position = new_state.clone_other_player().position
                    if board[our_player_new_position] == Field.Market: # Hier other_player, sind aber noch wir. Der andere Hase ist ja nach der Zugsimulation innerhalb der Simulation dran.
                        if our_player.position + 7 > new_state.clone_other_player().position: # wenn wir sehr viele Karotten haben, werden die Shopfelder sehr weit vorne NICHT hinzugefügt.
                            shop_moves.append(possible_moves[i])
                    
                    if board[our_player_new_position] == Field.Hedgehog:
                        hedgehog_moves.append(possible_moves[i])
            except:
                pass

        move_for_nearest_important_fields(possible_moves, board, shop_moves, hedgehog_moves, our_player)
    

        duration = time.time() - start_time
        duration2 = time.time() - start_time_2

        print ("Time needed before decision", duration)
        print ("2", duration2)
        
        if shop_moves != []:
            print ("Time needed for shop move", duration)
            return (random.choice(shop_moves))
        if hedgehog_moves != []:
            print ("Time needed for hedgehog move", duration)
            return (hedgehog_moves[0])
        if shop_moves == [] and hedgehog_moves == []:
            print ("Time needed for random move", duration)
            return (random.choice(possible_moves))

        print ("Time needed for random move, after the other 3 checks", duration)
        
        return (random.choice(possible_moves)) # Sicherheit, falls oben irgendwas nicht klappt
        
    
    
    # this method is called every time the server has sent a new game state update
    # this method should be implemented to keep the game state up to date
    def on_update(self, state: GameState) -> None:
        self.game_state = state




if __name__ == "__main__":
    Starter(logic=Logic())
    # if u wanna have more insights, u can set the logging level to debug:
    # Starter(logic=Logic(), log_level=logging.DEBUG)