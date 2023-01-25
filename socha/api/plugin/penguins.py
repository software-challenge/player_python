"""
This is the plugin for this year's game `Penguins`.
"""
import copy
import logging
import math
import warnings
from enum import Enum
from typing import List, Union, Optional


class Vector:
    """
    Represents a vector in the hexagonal grid. It can calculate various vector operations.
    """

    def __init__(self, d_x: int = 0, d_y: int = 0):
        """
        Constructor for the Vector class.

        :param d_x: The x-coordinate of the vector.
        :param d_y: The y-coordinate of the vector.
        """
        self.d_x = d_x
        self.d_y = d_y

    def magnitude(self) -> float:
        """
        Calculates the length of the vector.

        :return: The length of the vector.
        """
        return (self.d_x ** 2 + self.d_y ** 2) ** 0.5

    def dot_product(self, other: 'Vector'):
        """
        Calculates the dot product of two vectors.

        :param other: The other vector to calculate the dot product with.
        :return: The dot product of the two vectors.
        """
        return self.d_x * other.d_x + self.d_y * other.d_y

    def cross_product(self, other: 'Vector'):
        """
        Calculates the cross product of two vectors.

        :param other: The other vector to calculate the cross product with.
        :return: The cross product of the two vectors.
        """
        return self.d_x * other.d_y - self.d_y * other.d_x

    def scalar_product(self, scalar: int):
        """
        Extends the vector by a scalar.

        :param scalar: The scalar to extend the vector by.
        :return: The extended vector.
        """
        return Vector(self.d_x * scalar, self.d_y * scalar)

    def addition(self, other: 'Vector'):
        """
        Adds two vectors.

        :param other: The other vector to add.
        :return: The sum of the two vectors as a new vector object.
        """
        return Vector(self.d_x + other.d_x, self.d_y + other.d_y)

    def subtraction(self, other: 'Vector'):
        """
        Subtracts two vectors.

        :param other: The other vector to subtract.
        :return: The difference of the two vectors as a new vector object.
        """
        return Vector(self.d_x - other.d_x, self.d_y - other.d_y)

    def get_arc_tangent(self) -> float:
        """
        Calculates the arc tangent of the vector.

        :return: A radiant in float.
        """
        return math.atan2(self.d_y, self.d_x)

    def are_identically(self, other: 'Vector'):
        """
        Compares two vectors.

        :param other: The other vector to compare to.
        :return: True if the vectors are equal, false otherwise.
        """
        return self.d_x == other.d_x and self.d_y == other.d_y

    def are_equal(self, other: 'Vector'):
        """
        Checks if two vectors have the same magnitude and direction.

        :param other: The other vector to compare to.
        :return: True if the vectors are equal, false otherwise.
        """
        return self.magnitude() == other.magnitude() and self.get_arc_tangent() == other.get_arc_tangent()

    @property
    def directions(self) -> List['Vector']:
        """
        Gets the six neighbors of the vector.

        :return: A list of the six neighbors of the vector.
        """
        return [
            Vector(1, -1),  # UP RIGHT
            Vector(-2, 0),  # LEFT
            Vector(1, 1),  # DOWN RIGHT
            Vector(-1, 1),  # DOWN LEFT
            Vector(2, 0),  # Right
            Vector(-1, -1)  # UP LEFT
        ]

    def is_one_hex_move(self):
        """
        Checks if the vector points to a hexagonal field that is a direct neighbor.

        :return: True if the vector is a one hex move, false otherwise.
        """
        return abs(self.d_x) == abs(self.d_y) or (self.d_x % 2 == 0 and self.d_y == 0)

    def __repr__(self) -> str:
        """
        Returns the string representation of the vector.

        :return: The string representation of the vector.
        """
        return f"Vector({self.d_x}, {self.d_y})"

    def __eq__(self, other):
        """
        Overrides the default equality operator to check if two Vector objects are equal.

        :param other: The other Vector object to compare to.
        :return: True if the two Vector objects are equal, False otherwise.
        """
        if isinstance(other, Vector):
            return self.d_x == other.d_x and self.d_y == other.d_y
        return False


class Coordinate:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y

    def to_vector(self) -> Vector:
        """
        Converts the coordinate to a vector.
        """
        return Vector(d_x=self.x, d_y=self.y)

    def distance(self, other: 'Coordinate') -> float:
        """
        Calculates the distance between two coordinates.

        :param other: The other coordinate to calculate the distance to.
        :return: The distance between the two cartesian coordinates.
        """
        return self.to_vector().subtraction(other.to_vector()).magnitude()

    def add_vector(self, vector: Vector): ...

    def subtract_vector(self, vector: Vector): ...


class CartesianCoordinate(Coordinate):
    """
    Represents a coordinate in a normal cartesian coordinate system, that has been taught in school.
    This class is used to translate and represent a hexagonal coordinate in a cartesian and with that a 2D-Array.
    """

    def add_vector(self, vector: Vector) -> 'CartesianCoordinate':
        """
        Adds a vector to the cartesian coordinate.

        :param vector: The vector to add.
        :return: The new cartesian coordinate.
        """
        vector: Vector = self.to_vector().addition(vector)
        return CartesianCoordinate(x=vector.d_x, y=vector.d_y)

    def subtract_vector(self, vector: Vector) -> 'CartesianCoordinate':
        """
        Subtracts a vector from the cartesian coordinate.

        :param vector: The vector to subtract.
        :return: The new cartesian coordinate.
        """
        vector: Vector = self.to_vector().subtraction(vector)
        return CartesianCoordinate(x=vector.d_x, y=vector.d_y)

    def to_hex(self) -> 'HexCoordinate':
        """
        Converts the cartesian coordinate to a hex coordinate.

        :return: The hex coordinate.
        """
        return HexCoordinate(x=self.x * 2 + (1 if self.y % 2 == 1 else 0), y=self.y)

    def to_index(self) -> Optional[int]:
        """
        Converts the cartesian coordinate to an index.

        :return: The index or None if the coordinate is not valid.
        """
        if 0 <= self.x <= 7 and 0 <= self.y <= 7:
            return self.y * 8 + self.x
        return None

    @staticmethod
    def from_index(index: int, width: int, height: int) -> Optional['CartesianCoordinate']:
        """
        Converts a given index to a CartesianCoordinate.

        Args:
            index: The index to convert.
            width: The width of the grid.
            height: The height of the grid.

        Returns:
            Optional[CartesianCoordinate]: The CartesianCoordinate that corresponds to the given index, or None if the
            index is out of range.
        """
        if index < 0 or index >= width * height:
            raise IndexError(f"Index out of range. The index has to be 0 <= {index} < {width * height}")
        x = index % width
        y = index // width
        return CartesianCoordinate(x, y)

    def __repr__(self) -> str:
        return f"CartesianCoordinate({self.x}, {self.y})"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, CartesianCoordinate) and self.x == other.x and self.y == other.y


class HexCoordinate(Coordinate):
    """
    Represents a coordinate in a hexagonal coordinate system, that differs from the normal cartesian one.
    This class is used to represent the hexagonal game board.
    """

    def to_cartesian(self) -> CartesianCoordinate:
        """
        Converts the hex coordinate to a cartesian coordinate.

        :return: The cartesian coordinate.
        """
        return CartesianCoordinate(x=math.floor((self.x / 2 - (1 if self.y % 2 == 1 else 0)) + 0.5), y=self.y)

    def add_vector(self, vector: Vector) -> 'HexCoordinate':
        """
        Adds a vector to the hex coordinate.

        :param vector: The vector to add.
        :return: The new hex coordinate.
        """
        vector: Vector = self.to_vector().addition(vector)
        return HexCoordinate(x=vector.d_x, y=vector.d_y)

    def subtract_vector(self, vector: Vector) -> 'HexCoordinate':
        """
        Subtracts a vector from the hex coordinate.

        :param vector: The vector to subtract.
        :return: The new hex coordinate.
        """
        vector: Vector = self.to_vector().subtraction(vector)
        return HexCoordinate(x=vector.d_x, y=vector.d_y)

    def get_neighbors(self) -> List['HexCoordinate']:
        """
        Returns a list of all neighbors of the hex coordinate.

        :return: The list of neighbors.
        """
        return [self.add_vector(vector) for vector in self.to_vector().directions]

    def __repr__(self) -> str:
        return f"HexCoordinate({self.x}, {self.y})"

    def __eq__(self, other: object) -> bool:
        return isinstance(other, HexCoordinate) and self.x == other.x and self.y == other.y


class TeamEnum(Enum):
    ONE = "ONE"
    TWO = "TWO"


class Move:
    """
    Represents a move in the game.
    """

    def __init__(self, team_enum: TeamEnum, to_value: HexCoordinate, from_value: Optional[HexCoordinate]):
        """
        Args:
            team_enum: The team_enum that performs the move.
            to_value: The destination of the move.
            from_value: The origin of the move.
        """
        self.team_enum = team_enum
        self.from_value = from_value
        self.to_value = to_value

    def get_delta(self) -> float:
        """
        This method calculates and returns the difference in distance between the to_value and from_value properties
        of the Move object. If the from_value is not initialized, the distance is calculated between the to_value and
        itself.

        :return: The delta of the move as a float.
        """
        return self.to_value.distance(self.to_value if not self.from_value else self.from_value)

    def reversed(self):
        """
        This method returns a new Move object with the from_value and to_value properties reversed.
        If the current Move object is not initialized with a from_value, the method returns the current object.

        :return: The reversed move or the current move.
        """
        return self if not self.from_value else Move(team_enum=self.team_enum, to_value=self.from_value,
                                                     from_value=self.to_value)

    def __repr__(self):
        return f"Move(team={self.team_enum.value}, from={self.from_value}, to={self.to_value})"

    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Move) and self.to_value == __o.to_value and \
               (self.from_value is None or self.from_value == __o.from_value)


class Penguin:
    """
       The Penguin class represents a penguin object with a coordinate and a team_enum.
    """

    def __init__(self, coordinate: HexCoordinate, team_enum: TeamEnum):
        """
        Args:
           coordinate (HexCoordinate): The coordinate of the penguin on the game board.
           team_enum (TeamEnum): The team_enum that the penguin belongs to.
        """
        self.coordinate = coordinate
        self.team_enum = team_enum

    def get_distance(self, destination: HexCoordinate) -> float:
        """
        Calculates the distance from the current position to the given destination.

        Args:
            destination: The destination to calculate the distance to.

        Returns:
            float: The distance from the current position to the given destination.
        """
        return self.coordinate.distance(destination)

    def get_direction(self, destination: HexCoordinate) -> Vector:
        """
        Gets the direction of the move from the current coordinate to the given destination.

        Args:
            destination: The destination coordinate.

        Returns:
            Vector: The direction of the move.
        """
        return destination.subtract_vector(self.coordinate.to_vector()).to_vector()

    def __eq__(self, other):
        if not isinstance(other, Penguin):
            return False
        return self.coordinate == other.coordinate and self.team_enum == other.team_enum

    def __repr__(self):
        return f'Penguin({self.coordinate}, {self.team_enum.value})'


class Field:
    """
    Represents a field in the game.
    """

    def __init__(self, coordinate: HexCoordinate, penguin: Optional[Penguin], fish: int):
        """
        The Field represents a field on the game board.
        It says what state itself it has and where it is on the board.

        Args:
            coordinate:
            penguin:
            fish:
        """
        self.coordinate = coordinate
        self.penguin = penguin
        self.fish = fish

    def is_empty(self) -> bool:
        """
        :return: True if the field is has no fishes and no penguin, False otherwise.
        """
        return True if not self.penguin and self.fish == 0 else False

    def is_occupied(self) -> bool:
        """
        :return: True if the field is occupied by a penguin, False otherwise.
        """
        return True if self.penguin else False

    def get_fish(self) -> int:
        """
        :return: The amount of fish on the field, None if the field is occupied.
        """
        return self.fish

    def get_team(self) -> Union[TeamEnum, None]:
        """
        :return: The team_enum of the field if it is occupied by penguin, None otherwise.
        """
        return None if not self.penguin else self.penguin.team_enum

    def get_value(self) -> Union[TeamEnum, int]:
        """
        Returns the current value of the field. If the field has no penguin on it, it returns the number of fish on it,
        otherwise it returns the TeamEnum of the penguin on it.

        Returns:
            Union[TeamEnum, int]: The current value of the field.
        """
        return self.fish if not self.penguin else self.penguin.team_enum

    def get_distance(self, destination: HexCoordinate) -> float:
        """
        Calculates the distance from the current position to the given destination.

        Args:
            destination: The destination to calculate the distance to.

        Returns:
            float: The distance from the current position to the given destination.
        """
        return self.coordinate.distance(destination)

    def get_direction(self, destination: HexCoordinate) -> Vector:
        """
        Gets the direction of the move from the current coordinate to the given destination.

        Args:
            destination: The destination coordinate.

        Returns:
            Vector: The direction of the move.
        """
        return destination.subtract_vector(self.coordinate.to_vector()).to_vector()

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.coordinate == other.coordinate and self.penguin == other.penguin and self.fish == other.fish
        return False

    def __repr__(self):
        return f"Field({self.coordinate}, {self.penguin}, Fish({self.fish}))"


class Team:
    """
    The Team class is useful for storing and manipulating information about teams in the game. It allows you to
    easily create objects for each team_enum, keep track of their attributes, and compare them to their opponents.
    """

    def __init__(self, name: TeamEnum, fish: int, penguins: List[Penguin], moves: List[Move]):
        self.name = name
        self.fish = fish
        self.penguins = penguins
        self.moves = moves

    def team(self) -> TeamEnum:
        """
        :return: The team_enum object.
        """
        return self.name

    def get_penguins(self) -> List[Penguin]:
        return self.penguins

    def get_moves(self) -> List[Move]:
        return self.moves

    def color(self) -> str:
        """
        :return: The name of this team_enum.
        """
        if self.name == TeamEnum.ONE:
            return TeamEnum.ONE.value
        else:
            return TeamEnum.TWO.value

    @staticmethod
    def opponent() -> None:
        warnings.warn("Use the opponent method in GameState.")
        return None

    def __repr__(self) -> str:
        return f"Team(name={self.name}, fish={self.fish}, penguins={len(self.penguins)}, moves={len(self.moves)})"

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.name == other.name and self.fish == other.fish and self.penguins == other.penguins and \
                   self.moves == other.moves
        return False


class Board:
    """
    Class which represents a game board. Consisting of a two-dimensional array of fields.
    """

    def __init__(self, board: List[List[Field]]):
        """
        The Board shows the state where each field is, how many fish and which team is on each field.

        :param board: The game field as a two-dimensional array of fields.
        """
        self.board = board

    def get_empty_fields(self) -> List[Field]:
        """
        :return: A list of all empty fields.
        """
        fields: List[Field] = []
        for row in self.board:
            for field in row:
                if field.is_empty():
                    fields.append(field)
        return fields

    def is_occupied(self, coordinates: HexCoordinate) -> bool:
        """
        :param coordinates: The coordinates of the field.
        :return: True if the field is occupied, false otherwise.
        """
        return self.get_field(coordinates).is_occupied()

    def is_valid(self, coordinates: HexCoordinate) -> bool:
        """
        Checks if the coordinates are in the boundaries of the board.

        :param coordinates: The coordinates of the field.
        :return: True if the field is valid, false otherwise.
        """
        arrayCoordinates = coordinates.to_cartesian()
        return 0 <= arrayCoordinates.x < self.width() and 0 <= arrayCoordinates.y < self.height()

    def width(self) -> int:
        """
        :return: The width of the board.
        """
        return len(self.board[0])

    def height(self) -> int:
        """
        :return: The height of the board.
        """
        return len(self.board)

    def _get_field(self, x: int, y: int) -> Field:
        """
        Gets the field at the given coordinates.
        *Used only internally*

        :param x: The x-coordinate of the field.
        :param y: The y-coordinate of the field.
        :return: The field at the given coordinates.
        """
        return self.board[y][x]

    def get_field(self, position: HexCoordinate) -> Field:
        """
        Gets the field at the given position.

        :param position: The position of the field.
        :return: The field at the given position.
        :raise IndexError: If the position is not valid.
        """
        cartesian = position.to_cartesian()
        if self.is_valid(position):
            return self._get_field(cartesian.x, cartesian.y)

        raise IndexError(f"Index out of range: [x={cartesian.x}, y={cartesian.y}]")

    def get_field_or_none(self, position: HexCoordinate) -> Union[Field, None]:
        """
        Gets the field at the given position no matter if it is valid or not.

        :param position: The position of the field.
        :return: The field at the given position, or None if the position is not valid.
        """
        cartesian = position.to_cartesian()
        if self.is_valid(position):
            return self._get_field(cartesian.x, cartesian.y)
        return None

    def get_field_by_index(self, index: int) -> Field:
        """
        Gets the field at the given index. The index is the position of the field in the board.
        The field of the board is calculated as follows:

        - `x = index / width`
        - `y = index % width`
        - The index is 0-based. The index is calculated from the top left corner of the board.

        :param index: The index of the field.
        :return: The field at the given index.
        """
        return self.get_field(
            CartesianCoordinate.from_index(index=index, width=self.width(), height=self.height()).to_hex())

    def get_all_fields(self) -> List[Field]:
        """
        Gets all Fields of the board.

        :return: All Fields of the board.
        """
        return [field for row in self.board for field in row]

    def compare_to(self, other: 'Board') -> List[Field]:
        """
        Compares two boards and returns a list of the Fields that are different.

        :param other: The other board to compare to.
        :return: A list of Fields that are different or a empty list if the boards are equal.
        """
        if not isinstance(other, Board):
            raise TypeError("Can only compare to another Board object")

        fields = [self.board[x][y] for x in range(len(self.board)) for y in range(len(self.board[0]))
                  if self.board[x][y] != other.board[x][y]]
        return fields

    def contains(self, field: Field) -> bool:
        """
        Checks if the board contains the given field.

        :param field: The field to check for.
        :return: True if the board contains the field, False otherwise.
        """
        for row in self.board:
            if field in row:
                return True
        return False

    def contains_all(self, fields: List[Field]) -> bool:
        """
        Checks if the board contains all the given fields.

        :param fields: The fields to check for.
        :return: True if the board contains all the given fields, False otherwise.
        """
        if not fields:
            return False

        for field in fields:
            if not self.contains(field):
                return False
        return True

    def get_moves_in_direction(self, origin: HexCoordinate, direction: Vector, team_enum: TeamEnum) -> List[Move]:
        """
        Gets all moves in the given direction from the given origin.

        Args:
            origin: The origin of the move.
            direction: The direction of the move.
            team_enum: Team to make moves for.

        Returns:
                List[Move]: List of moves that can be made in the given direction from the given index,
                            for the given team_enum
        """
        if not self.get_field(origin).penguin or self.get_field(origin).penguin.team_enum != team_enum:
            return []

        moves = []
        for i in range(1, self.width()):
            destination = origin.add_vector(direction.scalar_product(i))
            if self._is_destination_valid(destination):
                moves.append(Move(team_enum=team_enum, from_value=origin, to_value=destination))
            else:
                break
        return moves

    def _is_destination_valid(self, field: HexCoordinate) -> bool:
        return self.is_valid(field) and not self.is_occupied(field) and not \
            self.get_field(field).is_empty()

    def possible_moves_from(self, position: HexCoordinate, team_enum: TeamEnum) -> List[Move]:
        """
        Returns a list of all possible moves from the given position. That are all moves in all hexagonal directions.

        Args:
            position: The position to start from.
            team_enum: A list of all possible moves from the given position.

        Returns:
            List[Move]: List of all possible moves that can be made from the given index, for the given team_enum

        Raises:
            IndexError: If the Index is out of range.
        """
        if not self.is_valid(position):
            raise IndexError(f"Index out of range: [x={position.x}, y={position.y}]")
        if not self.get_field(position).penguin or self.get_field(position).penguin.team_enum != team_enum:
            return []
        return [move for direction in Vector().directions for move in
                self.get_moves_in_direction(position, direction, team_enum)]

    def get_penguins(self) -> List[Penguin]:
        """
        Searches the board for all penguins.

        :return: A list of all Fields that are occupied by a penguin.
        """
        return [field.penguin for field in self.get_all_fields() if field.is_occupied()]

    def get_teams_penguins(self, team: TeamEnum) -> List[Penguin]:
        """
        Searches the board for all penguins of the given team_enum.

        :param team: The team_enum to search for.
        :return: A list of all coordinates that are occupied by a penguin of the given team_enum.
        """
        penguins = []
        for row in self.board:
            for field in row:
                if field.penguin and field.penguin.team_enum == team:
                    penguins.append(field.penguin)
        return penguins

    def get_most_fish(self) -> List[Field]:
        """
        Returns a list of all fields with the most fish.

        :return: A list of Fields.
        """

        fields = list(filter(lambda field_x: not field_x.is_occupied(), self.get_all_fields()))
        fields.sort(key=lambda field_x: field_x.get_fish(), reverse=True)
        for i, field in enumerate(fields):
            if field.get_fish() < fields[0].get_fish():
                fields = fields[:i]
        return fields

    def get_board_intersection(self, other: 'Board') -> List[Field]:
        """
        Returns a list of all fields that are in both boards.

        :param other: The other board to compare to.
        :return: A list of Fields.
        """
        return [field for field in self.get_all_fields() if field in other.get_all_fields()]

    def get_fields_intersection(self, other: List[Field]) -> List[Field]:
        """
        Returns a list of all fields that are in both list of Fields.

        :param other: The other list of Fields to compare to.
        :return: A list of Fields.
        """
        return [field for field in self.get_all_fields() if field in other]

    def _move(self, move: Move) -> 'Board':
        warnings.warn("'_move' is deprecated and will be removed in a future version. Use 'move' instead.",
                      DeprecationWarning)
        return self.move(move)

    def move(self, move: Move) -> 'Board':
        """
        Moves the penguin from the origin to the destination.
        **Please make sure that the move is correct, because this method will not check that.**
        If there is no Penguin to move, than this method will return the current state unchanged.

        :param move: The move to execute.
        :return: The new board with the moved penguin.
        """
        board_state = copy.deepcopy(self.board)
        updated_board = Board(board_state)
        moving_penguin = Penguin(team_enum=move.team_enum, coordinate=move.to_value)
        if move.from_value:
            if not self.get_field(move.from_value).penguin:
                logging.error(f"There is no penguin to move. Origin was: {self.get_field(move.from_value)}")
                return self
            origin_field_coordinate = move.from_value.to_cartesian()
            moving_penguin = board_state[origin_field_coordinate.y][origin_field_coordinate.x].penguin
            moving_penguin.coordinate = move.to_value
            board_state[origin_field_coordinate.y][origin_field_coordinate.x] = Field(coordinate=move.from_value,
                                                                                      penguin=None, fish=0)
        destination_field = updated_board.get_field(move.to_value)
        destination_field.penguin = moving_penguin
        destination_field.fish = 0
        return updated_board

    def pretty_print(self):
        print()
        for i, row in enumerate(self.board):
            if (i + 1) % 2 == 0:
                print(" ", end="")
            for field in row:
                if field.is_empty():
                    print("~", end=" ")
                elif field.is_occupied():
                    print(field.get_team().value[0], end=" ")
                else:
                    print(field.get_fish(), end=" ")
            print()
        print()

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.board == other.board
        return False


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

    def __init__(self, board: Board, turn: int, first_team: Team, second_team: Team, last_move: Optional[Move]):
        """
        Creates a new `GameState` with the given parameters.

        Args:
            board: The board of the game.
            turn: The turn number of the game.
            first_team: The team_enum that has the first turn.
            second_team: The team_enum that has the second turn.
            last_move: The last move made.
        """
        self.board = board
        self.turn = turn
        self.first_team = first_team
        self.second_team = second_team
        self.last_move = last_move
        self.round = int((self.turn + 1) / 2)
        self.current_team = self.current_team_from_turn(self.turn)
        self.current_pieces = self.current_team.get_penguins()
        self.possible_moves = self._get_possible_moves(self.current_team)

    def _get_possible_moves(self, current_team: Optional[Team]) -> List[Move]:
        """
        Gets all possible moves for the current team.
        That includes all possible moves from all Fields that are not occupied by a penguin from that team.

        :param current_team: The team to get the possible moves for.
        :return: A list of all possible moves from the current player's turn.
        """
        current_team = current_team or self.current_team
        moves = []
        if len(self.board.get_teams_penguins(current_team.name)) < 4:
            for x in range(self.board.width()):
                for y in range(self.board.height()):
                    field = self.board.get_field(CartesianCoordinate(x, y).to_hex())
                    if not field.is_occupied() and field.get_fish() == 1:
                        moves.append(
                            Move(team_enum=current_team.name, from_value=None,
                                 to_value=CartesianCoordinate(x, y).to_hex()))
        else:
            for piece in self.board.get_teams_penguins(current_team.name):
                moves.extend(self.board.possible_moves_from(piece.coordinate, current_team.name))
        return moves

    def current_team_from_turn(self, turn: int) -> Team:
        """
        Calculates the current team from the turn number and available moves.

        :return: The team that has the current turn.
        """
        current_team = self.first_team if turn % 2 == 0 else self.second_team
        possible_moves = self._get_possible_moves(current_team)
        if not possible_moves:
            current_team = self.second_team if turn % 2 == 0 else self.first_team
        return current_team

    def perform_move(self, move: Move) -> 'GameState':
        """
        Performs the given move on the current game state.

        Args:
            move: The move that has to be performed.

        Returns:
            GameState: The new state of the game after the move.
        """
        if self.is_valid_move(move) and self.current_team.name == move.team_enum:
            new_board = self.board.move(move)
            new_first_team = copy.deepcopy(self.first_team)
            new_second_team = copy.deepcopy(self.second_team)
            if self.current_team.name == TeamEnum.ONE:
                self._update_team(new_first_team, move, new_board)
            else:
                self._update_team(new_second_team, move, new_board)
            new_turn = self.turn + 1
            new_last_move = move
            return GameState(board=new_board, turn=new_turn, first_team=new_first_team, second_team=new_second_team,
                             last_move=new_last_move)
        else:
            logging.error(f"Performed invalid move while simulating: {move}")
            raise ValueError(f"Invalid move attempted: {move}")

    def _update_team(self, team: Team, move: Move, new_board: Board) -> None:
        """
        Helper function to update the given team when a move is performed.

        Args:
            team: The team that will be updated.
            move: The move that was performed.
            new_board: The updated board.
        """
        team.moves.append(move)
        adding_fish = self.board.get_field(move.to_value).get_fish()
        new_penguin = new_board.get_field(move.to_value).penguin
        teams_penguin = next(filter(lambda x: x.coordinate == move.from_value, team.penguins), None)
        if teams_penguin:
            teams_penguin.coordinate = new_penguin.coordinate
        else:
            team.penguins.append(new_penguin)
        team.fish += adding_fish

    def is_valid_move(self, move: Move) -> bool:
        """
        Checks if the given move is valid.
        
        :param move: The move to check.
        :return: True if the move is valid, False otherwise.
        """
        return move in self.possible_moves

    def opponent(self) -> Team:
        """
        Returns the opponent team of the current team.

        Returns:
            Team: The team which is the opponent of the current team.
        """
        if self.current_team == self.first_team:
            return self.second_team
        else:
            return self.first_team

    def __repr__(self):
        return f"GameState(turn={self.turn}, round={self.round}, first_team={self.first_team}, " \
               f"second_team={self.second_team}, last_move={self.last_move}, current_team={self.current_team})"
