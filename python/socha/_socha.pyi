from typing import Any, List, Optional, Set, Tuple


class PluginConstants:
    ROUND_LIMIT: int
    START_COAL: int
    MIN_SPEED: int
    MAX_SPEED: int
    FREE_ACC: int
    FREE_TURNS: int
    FINISH_POINTS: int
    POINTS_PER_PASSENGER: int
    POINTS_PER_SEGMENT: int
    NUMBER_OF_PASSENGERS: int
    SEGMENT_FIELDS_WIDTH: int
    SEGMENT_FIELDS_HEIGHT: int
    NUMBER_OF_SEGMENTS: int
    MAX_SPECIAL: int
    MIN_SPECIAL: int
    MAX_ISLANDS: int
    MIN_ISLANDS: int


class CartesianCoordinate:
    x: int
    y: int

    def __init__(cls, x: int, y: int) -> None: ...
    def to_cube(self) -> CubeCoordinates: ...
    def to_index(self) -> Optional[int]: ...
    def from_index(index: int) -> CartesianCoordinate: ...


class CubeCoordinates:
    q: int
    r: int
    s: int

    def __init__(cls, q: int, r: int) -> None: ...
    def coordinates(self) -> List[int]: ...
    def x(self) -> int: ...
    def y(self) -> int: ...
    def to_cartesian(self) -> CartesianCoordinate: ...
    def times(self, count: int) -> CubeCoordinates: ...
    def plus(self, other: CubeCoordinates) -> CubeCoordinates: ...
    def minus(self, other: CubeCoordinates) -> CubeCoordinates: ...
    def unary_minus(self) -> CubeCoordinates: ...
    def rotated_by(self, turns: int) -> CubeCoordinates: ...
    def distance_to(self, other: CubeCoordinates) -> int: ...
    def turn_count_to(self, target: CubeDirection) -> int: ...


class CubeDirection:
    Right: int
    DownRight: int
    DownLeft: int
    Left: int
    UpLeft: int
    UpRight: int

    def vector(self) -> CubeCoordinates: ...
    def angle(self) -> int: ...
    def with_neighbors(self) -> List[CubeDirection]: ...
    def opposite(self) -> CubeDirection: ...
    def turn_count_to(self, target: CubeDirection) -> int: ...
    def rotated_by(self, turns: int) -> CubeDirection: ...
    def ordinal(self) -> int: ...


class Move:
    actions: List[Accelerate | Advance | Push | Turn]

    def __init__(self, actions: List[Accelerate |
                 Advance | Push | Turn]) -> None: ...


class TeamEnum:
    One: int
    Two: int


class Ship:
    team: TeamEnum
    position: CubeCoordinates
    direction: CubeDirection
    speed: int
    coal: int
    passengers: int
    free_turns: int
    points: int
    free_acc: int
    movement: int

    def __init__(self, position: CubeCoordinates, team: TeamEnum,
                 direction: Optional[CubeDirection] = None,
                 speed: Optional[int] = None,
                 coal: Optional[int] = None,
                 passengers: Optional[int] = None,
                 free_turns: Optional[int] = None,
                 points: Optional[int] = None): ...

    def can_turn(self) -> bool: ...
    def max_acc(self) -> int: ...
    def accelerate_by(self, diff: int) -> None: ...
    def read_resolve(self) -> None: ...
    def resolve_direction(self, reverse: bool) -> None: ...
    def update_position(self, distance: int,
                        advance_info: AdvanceInfo) -> None: ...

    def __str__(self) -> str: ...


class AccelerationProblem:
    ZeroAcc: Any
    AboveMaxSpeed: Any
    BelowMinSpeed: Any
    InsufficientCoal: Any
    OnSandbank: Any

    def message(self) -> str: ...


class Accelerate:
    acc: int

    def __init__(self, acc: int) -> None: ...
    def perform(self, state: Any) -> Ship | BaseException: ...
    def __str__(self) -> str: ...


class AdvanceProblem:
    MovementPointsMissing: Any
    InsufficientPush: Any
    InvalidDistance: Any
    ShipAlreadyInTarget: Any
    FieldIsBlocked: Any
    MoveEndOnSandbank: Any

    def message(self) -> str: ...


class Advance:
    distance: int

    def __init__(self, distance: int) -> None: ...
    def perform(self, state: GameState) -> Ship | BaseException: ...


class PushProblem:
    MovementPointsMissing: Any
    SameFieldPush: Any
    InvalidFieldPush: Any
    BlockedFieldPush: Any
    SandbankPush: Any
    BackwardPushingRestricted: Any

    def message(self) -> str: ...


class Push:
    direction: CubeDirection

    def __init__(self, direction: CubeDirection) -> None: ...
    def perform(self, state: GameState) -> Tuple[Ship,
                                                 Ship] | BaseException: ...


class TurnProblem():
    RotationOnSandbankNotAllowed: Any
    NotEnoughCoalForRotation: Any
    RotationOnNonExistingField: Any

    def message(self) -> str: ...


class Turn:
    direction: CubeDirection

    def __init__(self, direction: CubeDirection) -> None: ...
    def perform(self, state: GameState) -> Ship | BaseException: ...

    def coal_cost(self, ship: Ship) -> int: ...


class Passenger:
    direction: CubeDirection
    passenger: int


class FieldType:
    Water: 0
    Island: 1
    Passenger: 2
    Goal: 3
    Sandbank: 4


class Field:
    field_type: FieldType
    passenger: Optional[Passenger]

    def __init__(field_type: FieldType,
                 passenger: Optional[Passenger]) -> None: ...

    def is_empty(self: Field) -> bool: ...
    def is_field_type(self: Field, field_type: FieldType) -> bool: ...


class Segment:
    direction: CubeDirection
    center: CubeCoordinates
    fields: List[List[Field]]

    def __init__(self, direction: CubeDirection,
                 center: CubeCoordinates, fields: List[List[Field]]) -> None: ...

    def tip(self) -> CubeCoordinates: ...
    def get(self, coordinates: CubeCoordinates) -> Optional[Field]: ...
    def set(self, coordinates: CubeCoordinates, field: Field) -> None: ...

    def local_to_global(
        self, coordinates: CubeCoordinates) -> CubeCoordinates: ...
    def global_to_local(
        self, coordinates: CubeCoordinates) -> CubeCoordinates: ...

    def contains(self, coordinates: CubeCoordinates) -> bool: ...
    def array_coords(self, coords: CubeCoordinates) -> CartesianCoordinate: ...
    def cube_coords(self, coords: CartesianCoordinate) -> CubeCoordinates: ...


class Board:
    segments: List[Segment]
    next_direction: CubeDirection

    def __init__(self, segments: List[Segment],
                 next_direction: CubeDirection) -> None: ...

    def get_segment(self, index: int) -> Optional[Segment]: ...
    def segment_with_index_at(
        self, coords: CubeCoordinates) -> Optional[tuple[int, Segment]]: ...

    def get(self, coords: CubeCoordinates) -> Optional[Field]: ...
    def does_field_have_current(self, coords: CubeCoordinates) -> bool: ...

    def does_field_have_stream(
        self, coords: CubeCoordinates) -> bool: ...

    def get_field_in_direction(
        self, direction: CubeDirection, coords: CubeCoordinates) -> Optional[Field]: ...

    def set_field_in_direction(
        self, direction: CubeDirection, coords: CubeCoordinates, field: Field) -> None: ...

    def get_coordinate_by_index(
        self, segment_index: int, x_index: int, y_index: int) -> CubeCoordinates: ...

    def segment_distance(self, coordinate1: CubeCoordinates,
                         coordinate2: CubeCoordinates) -> int: ...

    def segment_index(self, coordinate: CubeCoordinates) -> Optional[int]: ...

    def find_segment(
        self, coordinate: CubeCoordinates) -> Optional[Segment]: ...

    def neighboring_fields(
        self, coords: CubeCoordinates) -> List[Optional[Field]]: ...

    def effective_speed(self, ship: Ship) -> int: ...
    def is_sandbank(self, coords: CubeCoordinates) -> bool: ...
    def pickup_passenger(self, state: GameState) -> GameState: ...

    def pickup_passenger_at_position(
            self, pos: CubeCoordinates) -> Optional[Field]:
        """
            Picks up a passenger at the specified position using the CubeCoordinates.

            Args:
                pos (CubeCoordinates): The CubeCoordinates representing the position to check.

            Returns:
                Optional[Field]: The Field containing a passenger with a passenger count greater than 0,
                or None if no such Field is found in any adjacent direction.
        """

    def find_nearest_field_types(self, start_coordinates: CubeCoordinates,
                                 field_type: FieldType) -> Set[CubeCoordinates]:
        """
            A function to find the nearest field(s) of a specific type from a starting point in a hexagonal grid.

            Args:
                start_coordinates (CubeCoordinates): A CubeCoordinates object representing the starting point for the search.
                field_type (FieldType): A FieldType object representing the type of field being searched for.

            Returns:
                list of CubeCoordinates: A list of CubeCoordinates corresponding to the location of the nearest field(s) of the specified type.

            Note:
                This function will always return the coordinates of the nearest field(s) of the specified type, if such a field(s) exist.
                If multiple fields of the same type are at the same minimum distance, it returns all of them.
                If there isn't a field of the specified type or path to it, it will return an empty list.

            Examples:
                ```python
                from plugin import Board, CubeCoordinates, FieldType

                board = Board()
                board.find_nearest_field_types(CubeCoordinates(0, 0), FieldType.Water)
                ```
        """


class TeamPoints:
    ship_points: int
    coal_points: int
    finish_points: int


class AdvanceInfo:
    costs: int
    problem: AdvanceProblem

    def cost_until(self, distance: int) -> int: ...
    def advances(self, distance: Optional[int]) -> List[Advance]: ...
    def distance(self) -> int: ...


class GameState:
    board: Board
    turn: int
    current_ship: Ship
    other_ship: Ship
    last_move: Optional[Move]

    def __init__(self, board: Board, turn: int, current_ship: Ship,
                 other_ship: Ship, last_move: Optional[Move]) -> None: ...

    def determine_ahead_team(self) -> Ship: ...
    def ship_advance_points(self, ship: Ship) -> int: ...
    def calculate_points(self, ship: Ship) -> int: ...
    def is_current_ship_on_current(self) -> bool: ...
    def perform_action(self, action: Accelerate | Advance |
                       Push | Turn) -> GameState: ...

    def perform_move(self, move: Move) -> GameState: ...
    def perform_move_unchecked(self, move: Move) -> GameState: ...
    def advance_turn(self) -> GameState: ...
    def effective_speed(self, ship: Ship) -> int: ...
    def remove_passenger_at(self, coords: CubeCoordinates) -> bool: ...
    def pick_up_passenger_current_ship(self) -> None: ...
    def pick_up_passenger_other_ship(self) -> None: ...
    def ship_advance_points(self, ship: Ship) -> int: ...
    def ship_points(self, ship: Ship) -> int: ...
    def must_push(self) -> bool: ...
    def check_ship_advance_limit(self, ship: Ship) -> AdvanceInfo: ...
    def calculate_advance_info(self, start: CubeCoordinates,
                               direction: CubeDirection, max_movement_points: int) -> AdvanceInfo: ...

    def possible_accelerations(self) -> List[Accelerate]: ...
    def possible_pushes(self) -> List[Push]: ...
    def possible_turns(self) -> List[Turn]: ...
    def possible_advances(self) -> List[Advance]: ...
    def sandbank_advances_for(self, ship: Ship) -> Optional[List[Advance]]: ...

    def possible_moves(self) -> List[Move]: ...

    def possible_action_comb(self, current_state: GameState,
                             current_actions: List[Accelerate | Advance | Push | Turn],
                             depth: int,
                             max_depth: int) -> List[List[Accelerate | Advance | Push | Turn]]: ...

    def possible_actions(self, rank: int) -> List[Accelerate |
                                                  Advance | Push | Turn]: ...

    def coal_for_action(self, action: Accelerate | Advance |
                        Push | Turn) -> int: ...

    def can_move(self) -> bool: ...
    def is_over(self) -> bool: ...
    def is_winner(self, ship: Ship) -> bool: ...
    def get_points_for_team(self, ship: Ship) -> TeamPoints: ...
