# Nicht alle Imports werden aktuell verwendet.
# Diese können in Zukunft aber sehr nützlich sein
# und zeigen außerdem alle Funktionalitäten der Software-Challenge Python API.
import math
import random
import time
from typing import Optional, Tuple
from socha import (
    Coordinate,
    Vector,
    Direction,
    FieldType,
    TeamEnum,
    Board,
    Move,
    GameState,
    RulesEngine,
)
from socha.api.networking.game_client import IClientHandler
from socha.starter import Starter


class Logic(IClientHandler):
    game_state: GameState

    # Diese Methode wird immer aufgerufen, wenn der Spielserver einen Zug vom Client anfordert.
    # Sie muss implementiert sein, weil der Computerspieler sonst disqualifiziert wird.
    def calculate_move(self) -> Move:
        return random.choice(self.game_state.possible_moves())

    # Diese Methode wird jedes Mal aufgerufen, wenn der Server einen neunen Spielstand bereitstellt.
    # Sie muss implentiert sein, damit die Spielstand Instanz auf dem neusten Stand bleibt.
    def on_update(self, state: GameState) -> None:
        self.game_state = state

    # Die Klasse IClientHandler hält noch weitere Methoden, die durch bestimmte Aktionen des Servers ausgeführt werden.
    # Weitere Informationen dazu gibt es in der verlinkten Dokumentation unter dem Submodul socha.api.networking.game_client.

if __name__ == "__main__":
    Starter(logic=Logic())
    # Wenn man mehr Debug Informationen aus dem Hintergrund der API haben möchte, kann das Log-Level auf debug gesetzt werden:
    # Starter(logic=Logic(), log_level=logging.DEBUG)
