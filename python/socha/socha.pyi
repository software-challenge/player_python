from typing import List, Type
from api.protocol.protocol import Result

class Starter:
    """
    When this is called, the client will try to connect to the server and join a game.
    When successful, the client will start the loop and call the on_update and calculate_move methods,
    if the server sends updates.
    """

    def __init__(self, logic: Type[IClientHandler], host: str = "localhost", port: int = 13050, reservation: str = None,
                 room_id: str = None, survive: bool = False, log: bool = False, verbose: bool = False):
        """
        All these arguments can be overwritten, when parsed via start arguments,
        or you initialize this class with the desired VALUES.

        :param logic: Your logic the client will call, if moves are requested.
        :param host: The host that the client should connect to.
        :param port: The port of the host.
        :param reservation: Reservation code for a prepared game.
        :param room_id: Room Id the client will try to connect.
        :param survive: If True the client will keep running, even if the connection to the server is terminated.
        :param log: If True the client will write a log file to the current directory.
        :param verbose: Verbose option for logging.
        """
        ...

class BitBoard:
    """
    Class which represents a game board. Consisting of seven 64-bit unsigned integers.
    """
    def __init__(self, one: int, two: int, fish_0: int, fish_1: int, fish_2: int, fish_3: int, fish_4: int) -> None:
        """
        This board represents a state of the game in seven different bitboards. Each bitboard represents a different case:

        :param one: A bitboard representing the positions of the penguins of the first team.
        :param two: A bitboard representing the positions of the penguins of the second team.
        :param fish_0: A bitboard representing the positions 0 fish fields.
        :param fish_1: A bitboard representing the positions 1 fish fields.
        :param fish_2: A bitboard representing the positions 2 fish fields.
        :param fish_3: A bitboard representing the positions 3 fish fields.
        :param fish_4: A bitboard representing the positions 4 fish fields.
        """
    def equivalence(self, other: BitBoard) -> bool:
        """
        Checks if two boards are equivalent.

        :param other: The other board to compare with.
        :return: True if the two boards are equivalent, false otherwise.
        """
    def is_empty(self) -> bool:
        """
        Checks if the board is is_empty.

        :return: True if the board is is_empty, false otherwise.
        """
    def intersection(self, other: BitBoard) -> BitBoard:
        """
        Calculates the intersection of two boards.

        :param other: The other board to intersect with.
        :return: The intersection of the two boards.
        """
    def union(self, other: BitBoard) -> BitBoard:
        """
        Calculates the union of two boards.

        :param other: The other board to union with.
        :return: The union of the two boards.
        """
    def difference(self, other: BitBoard) -> BitBoard:
        """
        Calculates the difference of two boards.

        :param other: The other board to difference with.
        :return: The difference of the two boards.
        """
    def disjoint(self, other: BitBoard) -> bool:
        """
        Checks if two boards are disjoint.

        :param other: The other board to check with.
        :return: True if the two boards are disjoint, false otherwise.
        """
    def complement(self) -> BitBoard:
        """
        Calculates the complement of the board.

        :return: The complement of the board.
        """
    def implication(self, other: BitBoard) -> BitBoard:
        """
        Calculates the implication of two boards.

        :param other: The other board to imply with.
        :return: The implication of the two boards.
        """
    def exclusive_or(self, other: BitBoard) -> BitBoard:
        """
        Calculates the exclusive or of two boards.

        :param other: The other board to exclusive or with.
        :return: The exclusive or of the two boards.
        """
    def update(self, other: Move) -> None:
        """
        Updates the board with a move.

        :param other: The move to update the board with.
        """
    def set_field(self, field: Field) -> None:
        """
        Sets a field on the board.

        :param field: The field to set.
        """
    def get_field(self, index: int) -> Field:
        """
        Gets a field from the board.

        :param index: The index of the field.
        :return: The field from the board.
        """
    def get_penguin(self, coordinate: HexCoordinate) -> int:
        """
        Gets the penguin on a coordinate.

        :param coordinate: The coordinate of the penguin.
        :return: The penguin on the coordinate.
        """
    def get_fish(self, index: int) -> int:
        """
        Gets the number of fish on a field.
        
        :param index: The index of the field.
        :return: The number of fish on the field.
        """
    def get_empty_bits(self) -> int:
        """
        Gets the number of is_empty bits on the board.

        :return: The number of is_empty bits on the board.
        """
    def is_occupied(self, index: int) -> bool:
        """
        Checks if a field is occupied by a penguin.

        :param index: The index of the field.
        :return: True if the field is occupied, false otherwise.
        """
    def is_valid(self, index: int) -> bool:
        """
        Checks if a field is valid.

        :param index: The index of the field.
        :return: True if the field is valid, false otherwise.
        """
    def contains_field(self, index: int) -> bool:
        """
        Checks if the board contains a field.

        :param index: The index of the field.
        :return: True if the board contains the field, false otherwise.
        """
    def contains(self, indexes: List[int]) -> bool:
        """
        Checks if the board contains a list of fields.

        :param indexes: The indexes of the fields.
        :return: True if the board contains the fields, false otherwise.
        """
    def is_team(self, team: TeamEnum, index: int) -> bool:
        """
        Checks if a field is occupied by a penguin of a team.

        :param team: The team to check.
        :param index: The index of the field.
        :return: True if the field is occupied by a penguin of the team, false otherwise.
        """
    def get_coordinates(self, bitboard: int) -> List[HexCoordinate]:
        """
        Gets the coordinates of a bitboard.

        :param bitboard: The bitboard to get the coordinates from.
        :return: The coordinates of the bitboard.
        """
    def get_bit_coordinate(self, field: BitBoard) -> HexCoordinate:
        """
        With the passed BitBoard, which should have only one bit set, 
        a field is returned which is also set on the representative BitBoards.

        :param field: The field to get the coordinate from.
        :return: The coordinate of the field.
        """
    def get_directive_moves(self, index:int, direction: Vector, team: TeamEnum) -> List[Move]:
        """
        Gets all possible moves in a direction. If a field is occupied by a penguin or has no fish, the direction ends.

        :param index: The index of the field to start from.
        :param direction: The direction to get the moves in.
        :param team: The team to get the moves for.
        :return: A list of all possible moves in the direction.
        """
    def possible_moves_from(self, index: int, team: TeamEnum) -> List[Move]:
        """
        Gets all possible moves from a field.

        :param index: The index of the field to get the moves from.
        :param team: The team to get the moves for.
        :return: A list of all possible moves from the field.
        """
class Board:
    """
    Class which represents a game board. Consisting of a two-dimensional array of fields.
    """
    def __init__(self, board: List[List[Field]]) -> None:
        """
        Constructor for the Board class.

        :param board: The two-dimensional array of fields.
        """
    def get_fields(self) -> List[List[Field]]:
        """
        Gets the two-dimensional array of fields.

        :return: The two-dimensional array of fields.
        """
    def get_empty_fields(self) -> List[Field]:
        """
        Gets all empty fields on the board.

        :return: A list of all empty fields on the board.
        """
    def is_occupied(self, field: Field) -> bool:
        """
        Checks if a field is occupied by a penguin.

        :param field: The field to check.
        :return: True if the field is occupied, false otherwise.
        """
    def is_valid(self, coordinate: HexCoordinate) -> bool:
        """
        Checks if a coordinate is valid.

        :param coordinate: The coordinate to check.
        :return: True if the coordinate is valid, false otherwise.
        """
    def get_field(self, coordinate: HexCoordinate) -> Field:
        """
        Gets a field from the board.

        :param coordinate: The coordinate of the field.
        :return: The field from the board.
        """
    def contains_field(self, field: Field) -> bool:
        """
        Checks if the board contains a field.

        :param field: The field to check.
        :return: True if the board contains the field, false otherwise.
        """
    def contains(self, fields: List[Field]) -> bool:
        """
        Checks if the board contains a list of fields.

        :param fields: The fields to check.
        :return: True if the board contains the fields, false otherwise.
        """
    def get_directive_moves(self, coordinate: HexCoordinate, direction: Vector, team: TeamEnum) -> List[Move]:
        """
        Gets all possible moves in a direction. If a field is occupied by a penguin or has no fish, the direction ends.

        :param coordinate: The coordinate to start from.
        :param direction: The direction to get the moves in.
        :param team: The team to get the moves for.
        :return: A list of all possible moves in the direction.
        """
    def possible_moves_from(self, coordinate: HexCoordinate, team: TeamEnum) -> List[Move]:
        """
        Gets all possible moves from a field.

        :param coordinate: The coordinate to get the moves from.
        :param team: The team to get the moves for.
        :return: A list of all possible moves from the field.
        """
    def get_penguins(self, team: TeamEnum) -> List[Penguin]:
        """
        Gets all penguins of a team.

        :param team: The team to get the penguins from.
        :return: A list of all penguins of the team.
        """
    def get_teams_penguins(self, team: TeamEnum) -> List[Penguin]:
        """
        Gets all penguins of a team.

        :param team: The team to get the penguins from.
        :return: A list of all penguins of the team.
        """

class CartesianCoordinate:
    """
    Represents a coordinate in a normal cartesian coordinate system, that has been taught in school.
    This class is used to translate and represent a hexagonal coordinate in a cartesian and with that a 2D-Array.
    """
    def __init__(self, x: int, y: int) -> None:
        """
        Constructor for the CartesianCoordinate class.

        :param x: The x-coordinate on a cartesian coordinate system.
        :param y: The y-coordinate on a cartesian coordinate system.
        """
    def to_vector(self) -> Vector:
        """
        Converts the CartesianCoordinate to a Vector.

        :return: The Vector representing the CartesianCoordinate.
        """
    def add_vector(self, vector: Vector) -> CartesianCoordinate:
        """
        Adds a vector to the CartesianCoordinate.

        :param vector: The vector to add.
        :return: The new CartesianCoordinate.
        """
    def subtract_vector(self, vector: Vector) -> CartesianCoordinate:
        """
        Subtracts a vector from the CartesianCoordinate.

        :param vector: The vector to subtract.
        :return: The new CartesianCoordinate.
        """
    def distance(self, other: CartesianCoordinate) -> int:
        """
        Calculates the distance between two CartesianCoordinates.

        :param other: The other CartesianCoordinate.
        :return: The distance between the two CartesianCoordinates.
        """
    def to_hex(self) -> HexCoordinate:
        """
        Converts the CartesianCoordinate to a HexCoordinate.

        :return: The HexCoordinate representing the CartesianCoordinate.
        """
    @staticmethod
    def from_index(index: int) -> CartesianCoordinate:
        """
        Converts an index to a CartesianCoordinate.

        :param index: The index to convert.
        :return: The CartesianCoordinate representing the index.
        """

class Field:
    """
    Represents a field in the game.
    """
    def __init__(self, coordinate: HexCoordinate, penguin: Penguin | None, fish: int) -> None:
        """
        The Field represents a field on the game board.
        It says what state itself it has and where it is on the board.

        :param coordinate: The coordinate of the field.
        :param penguin: The penguin on the field.
        :param fish: The number of fish on the field.
        """
    def is_empty(self) -> bool:
        """
        Checks if the field is is_empty.

        :return: True if the field is is_empty, false otherwise.
        """
    def has_penguin(self) -> bool:
        """
        Checks if the field has a penguin.

        :return: True if the field has a penguin, false otherwise.
        """
    def get_team(self) -> TeamEnum | None:
        """
        Gets the team of the penguin on the field.

        :return: The team of the penguin on the field, if there is no penguin, None is returned instead.
        """

class GameState:
    """
       A `GameState` contains all information, that describes the game state at a given time, that is, between two game
       moves.
       This includes:
            - the board
            - a consecutive turn number (round & turn) and who's turn it is
            - the team that has started the game
            - the number of fishes each player has
            - the last move made
       The `GameState` is thus the central object through which all essential information of the current game can be
       accessed.
       Therefore, for easier handling, it offers further aids, such as:
            - a method to calculate available moves
            - a method to perform a move for simulating future game states
       The game server sends a new copy of the `GameState` to both participating players after each completed move,
       describing the then current state.
    """
    def __init__(self, welcome_message: WelcomeMessage, start_team: Team, board: Board,
                      round: Progress, score: Score, last_move: Move | None) -> None:
        """
        Creates a new `GameState` with the given parameters.

        :param welcome_message: The welcome message, which contains the client's team.
        :param start_team: The team that starts the game.
        :param board: The board of the game.
        :param round: The round and turn number.
        :param score: The current scores of the teams.
        :param last_move: The last move made.
        """
    def current_team(self) -> TeamEnum:
        """
        Gets the team which turn it is, based on possible moves.

        :return: The team which turn it is.
        """

    def get_opponent(self) -> TeamEnum:
        """
        Gets the opponent of current team.

        :return: The opponent of current team.
        """

    def possible_moves(self) -> List[Move]:
        """
        Calculates all possible moves for the current team.

        :return: A list of all possible moves.
        """

class HexCoordinate:
    """
    Represents a coordinate in a hexagonal coordinate system, that differs from the normal cartesian one.
    This class is used to represent the hexagonal game board.
    """
    def __init__(self, x: int, y: int) -> None:
        """
        Constructor for the HexCoordinate class.

        :param x: The x-coordinate on a hexagonal coordinate system.
        :param y: The y-coordinate on a hexagonal coordinate system.
        """
    def to_cartesian(self) -> CartesianCoordinate:
        """
        Converts the coordinate to a cartesian coordinate.

        :return: The cartesian coordinate.
        """
    def to_vector(self) -> Vector:
        """
        Converts the coordinate to a vector.

        :return: The vector.
        """
    def get_neighbors(self) -> List[HexCoordinate]:
        """
        Gets the neighbors of the coordinate.

        :return: A list of the neighbors.
        """
    def add_vector(self, vector: Vector) -> HexCoordinate:
        """
        Adds a vector to the coordinate.

        :param vector: The vector to add.
        :return: The new coordinate.
        """
    def subtract_vector(self, vector: Vector) -> HexCoordinate:
        """
        Subtracts a vector from the coordinate.

        :param vector: The vector to subtract.
        :return: The new coordinate.
        """
    def distance(self, other: HexCoordinate) -> int:
        """
        Calculates the distance between two coordinates.

        :param other: The other coordinate.
        :return: The distance between the two coordinates.
        """

class IClientHandler:
    def calculate_move(self) -> Move:
        """
        Calculates a move that the logic wants the server to perform in the game room.
        """
    def on_update(self, state: GameState):
        """
        If the server send a update on the current state of the game this method is called.
        :param state: The current state that server sent.
        """
    def on_game_over(self, roomMessage: Result):
        """
        If the game has ended the server will send a result message.
        This method will called if this happens.

        :param roomMessage: The Result the server has sent.
        """
    def on_error(self, logMessage: str):
        """
        If error occurs,
        for instance when the logic sent a move that is not rule conform,
        the server will send an error message and closes the connection.
        If this happens, this method is called.

        :param logMessage: The message, that server sent.
        """
    def on_room_message(self, data):
        """
        If the server sends a message that cannot be handelt by anny other method,
        this will be called.

        :param data: The data the Server sent.
        """
    def on_game_prepared(self, message):
        """
        If the game has been prepared by the server this method will be called.

        :param message: The message that server sends with the response.
        """
    def on_game_joined(self, room_id):
        """
        If the client has successfully joined a game room this method will be called.

        :param room_id: The room id the client has joined.
        """
    def on_game_observed(self, message):
        """
        If the client successfully joined as observer this method will be called.

        :param message: The message that server sends with the response.
        """

class Move:
    """
    Represents a move in the game.
    """
    def __init__(self, _from: HexCoordinate | None, to: HexCoordinate, team: TeamEnum):
        """
        Constructor for the Move class.

        :param _from: The field to move from, if it is the first move, this is None.
        :param to: The field to move to.
        """
    def delta(self) -> int:
        """
        Calculates the delta of the move.

        :return: The delta of the move.
        """
    def reverse(self) -> Move:
        """
        Reverses the move.

        :return: The reversed move.
        """

class Penguin:
    """
    Represents a penguin in the game.
    """
    def __init__(self, position: HexCoordinate, team: TeamEnum) -> None:
        """
        Constructor for the Penguin class.

        :param position: The position of the penguin.
        :param team: The team of the penguin.
        """

class Progress:
    """
    Represents the progress of a game.
    """
    def __init__(self, round: int, turn: int) -> None:
        """
        Constructor for the Progress class.

        :param round: The current round.
        :param turn: The current turn.
        """

class Score:
    """
    Represents the score of a game.
    """
    def __init__(self, team_one: int, team_two: int) -> None:
        """
        Constructor for the Score class.

        :param team_one: The score of team one.
        :param team_two: The score of team two.
        """

class Starter:
    """
    When this is called, the client will try to connect to the server and join a game.
    When successful, the client will start the loop and call the on_update and calculate_move methods,
    if the server sends updates.
    """
    def __init__(self, logic: IClientHandler, host: str = "localhost", port: int = 13050, reservation: str = None,
                 room_id: str = None, survive: bool = False, log: bool = False, verbose: bool = False):
        """
        All these arguments can be overwritten, when parsed via start arguments,
        or you initialize this class with the desired VALUES.

        :param logic: Your logic the client will call, if moves are requested.
        :param host: The host that the client should connect to.
        :param port: The port of the host.
        :param reservation: Reservation code for a prepared game.
        :param room_id: Room Id the client will try to connect.
        :param survive: If True the client will keep running, even if the connection to the server is terminated.
        :param log: If True the client will write a log file to the current directory.
        :param verbose: Verbose option for logging.
        """
    
class Team:
    """
    Represents a team in the game.
    """
    def __init__(self, name: TeamEnum, penguins: List[Penguin], fish: int) -> None:
        """
        Constructor for the Team class.
        A Team can be either yourself or your opponent.
        """
    def opponent(self) -> Team:
        """
        Gets the opponent of the team.

        :return: The opponent of the team.
        """

class TeamEnum:
    """
    Holds both teams for identification.
    It only stores the name of both teams.
    """

class Vector:
    """
    Represents a vector in the hexagonal grid. It can calculate various vector operations.
    """
    def __init__(self, x: int, y: int) -> None: ...
    @staticmethod
    def neighbors() -> List[Vector]: ...
    def magnitude(self) -> float: ...
    def normalize(self) -> Vector: ...
    def dot(self, other: Vector) -> int: ...
    def cross(self, other: Vector) -> int: ...
    def angle(self, other: Vector) -> float: ...
    def scalar(self, scalar: int) -> Vector: ...
    def add(self, other: Vector) -> Vector: ...
    def sub(self, other: Vector) -> Vector: ...
    def eq(self, other: Vector) -> bool: ...

class WelcomeMessage:
    """
    Represents the welcome message the server sends.
    """
    def __init__(self, team: TeamEnum) -> None:
        """
        Constructor for the WelcomeMessage class.

        :param team: The team the client is assigned to.
        """
