from enum import Enum
from typing import Dict, List, Optional


class Card:
    FallBack: Card
    HurryAhead: Card
    EatSalad: Card
    SwapCarrots: Card

    def __init__(self) -> None: ...

    def moves(self) -> bool: ...

    def play(self, state: GameState) -> None: ...


class Advance:
    def __init__(self, distance: int, cards: List[Card]): ...

    def perform(self, state: GameState) -> None: ...


class EatSalad:
    def __init__(self) -> None: ...

    def perform(self, state: GameState) -> None: ...


class ExchangeCarrots:
    def __init__(self, value: int) -> None: ...

    def perform(self, state: GameState) -> None: ...


class FallBack:
    def perform(self, state: GameState) -> None: ...


class Field(Enum):
    Position1: int
    Position2: int
    Hedgehog: int
    Salad: int
    Carrots: int
    Hare: int
    Market: int
    Goal: int
    Start: int


class Board:
    track: list[Field]

    def __init__(self, track: list[Field]) -> None: ...

    def get_field(self, index: int) -> Optional[Field]: ...

    def find_field(self, field: Field, start: int,
                   end: int) -> Optional[int]: ...

    def get_previous_field(
        self, field: Field, index: int) -> Optional[int]: ...

    def get_next_field(self, field: Field, index: int) -> Optional[int]: ...


class TeamEnum(Enum):
    One: int
    Two: int

    def __repr__(self) -> str: ...


class Hare:
    team: TeamEnum
    position: int
    salads: int
    carrots: int
    salad_eaten: bool
    cards: List[Card]

    def __init__(
        self,
        team: TeamEnum,
        cards: Optional[List[Card]] = None,
        carrots: Optional[int] = None,
        salads: Optional[int] = None,
        salad_eaten: Optional[bool] = None,
        position: Optional[int] = None
    ) -> None: ...

    def is_in_goal(self) -> bool: ...

    def can_enter_goal(self) -> bool: ...

    def advance_by(self, distance: int) -> None: ...

    def consume_carrots(self, carrots: int) -> None: ...

    def eat_salad(self) -> None: ...


class Move:
    action: Advance | EatSalad | ExchangeCarrots | FallBack

    def __init__(self, action: Advance | EatSalad | ExchangeCarrots | FallBack) -> None:
        ...

    def __repr__(self) -> str:
        ...


class GameState:
    board: Board
    turn: int
    moves: Dict[int, Move]

    def __init__(self, board: Board, turn: int, player_one: Hare, player_two: Hare, moves: Dict[int, Move]):
        self.board = board
        self.turn = turn
        self.player_one = player_one
        self.player_two = player_two
        self.moves = moves

    def get_current_player(self) -> Hare:
        ...

    def set_current_player(self, player: Hare) -> None:
        ...

    def get_other_player(self, player: Hare) -> Hare:
        ...

    def set_other_player(self, player: Hare) -> None:
        ...

    def is_ahead(self, player: Hare) -> bool:
        ...

    def can_exchange_carrots(self, player: Hare, count: int) -> bool:
        ...

    def must_eat_salad(self, player: Hare) -> bool:
        ...

    def eat_salad(self, player: Hare) -> None:
        ...

    def get_fall_back(self, player: Hare) -> Optional[int]:
        ...

    def move_to_field(self, player: Hare, new_position: int) -> None:
        ...

    def can_advance_to(self, new_position: int, player: Hare) -> None:
        ...
