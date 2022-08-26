"""
This is the plugin for this year's game `Penguins`.
"""
import math


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
        return math.degrees(math.atan2(self.d_y, self.d_x))

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
    def directions(self) -> list['Vector']:
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
        Checks if the vector is a one hex move.
        :return: True if the vector is a one hex move, false otherwise.
        """
        return abs(self.d_x) == abs(self.d_y) or (self.d_x % 2 == 0 and self.d_y == 0)

    def to_coordinates(self) -> 'Coordinate':
        """
        Converts the vector to coordinate object.
        :return:    The coordinate object.
        """
        return Coordinate(self.d_x, self.d_y, is_double=True)

    def __str__(self) -> str:
        """
        Returns the string representation of the vector.
        :return: The string representation of the vector.
        """
        return f"Vector({self.d_x}, {self.d_x})"


class Coordinate:
    """
    Representation of a coordination system in the hexagonal grid.
    """

    def __init__(self, x: int, y: int, is_double: bool = True):
        """
        Constructor for the Coordinates class.
        :param x: The x-coordinate of the coordination system.
        :param y: The y-coordinate of the coordination system.
        :param is_double: Determines if the coordinate is in double hex format. Default is True.
        """
        self.x = x
        self.y = y
        self.is_double = is_double

    def add_vector(self, vector: Vector) -> 'Coordinate':
        """
        Adds a vector to the coordinate.
        :param vector: The vector to add.
        :return: The new coordinate.
        """

        return self.get_vector().addition(vector).to_coordinates() if self.is_double else \
            self.get_double_hex().get_vector().addition(vector).to_coordinates().get_array()

    def subtract_vector(self, vector: Vector) -> 'Coordinate':
        """
        Subtracts a vector from the coordinate.
        :param vector: The vector to subtract.
        :return: The new coordinate.
        """
        return self.get_vector().subtraction(vector).to_coordinates()

    def get_distance(self, other: 'Coordinate') -> float:
        """
        Calculates the distance between two coordinates.
        :param other: The other coordinate to calculate the distance to.
        :return: The distance between the two coordinates as Vector object.
        """
        return self.get_vector().subtraction(other.get_vector()).magnitude()

    def get_vector(self) -> Vector:
        """
        Gets the vector from the coordinate to the origin.
        :return: The vector from the coordinate to the origin.
        """
        return Vector(self.x, self.y)

    def get_hex_neighbors(self) -> list[Vector]:
        """
        Gets the six neighbors of the coordinate.
        :return: A list of the six neighbors of the coordinate.
        """
        ...

    def __array_to_double_hex(self) -> 'Coordinate':
        """
        Converts the coordinate to double hex coordinates.
        :return: The double hex coordinates.
        """
        return Coordinate(self.x * 2 + (1 if self.y % 2 == 1 else 0), self.y, True)

    def __double_hex_to_array(self) -> 'Coordinate':
        """
        Converts the double hex coordinates to coordinate.
        :return: The coordinate.
        """
        return Coordinate(math.floor((self.x / 2 - (1 if self.y % 2 == 1 else 0)) + 0.5), self.y, False)

    def get_array(self) -> 'Coordinate':
        """
        Checks if the coordinate is an array or double hex coordinate.
        :return: Self if the coordinate is an array, __double_hex_to_array if the coordinate is a double hex coordinate.
        """
        return self if not self.is_double else self.__double_hex_to_array()

    def get_double_hex(self) -> 'Coordinate':
        """
        Checks if the coordinate is a double hex coordinate.
        :return: Self if the coordinate is a double hex coordinate, __double_hex_to_array if the coordinate is an array.
        """
        return self if self.is_double else self.__array_to_double_hex()

    def __str__(self) -> str:
        return f"Coordinate[{self.x}, {self.y}]"


class Move:
    """
    Represents a move in the game.
    """

    def __init__(self, to_value: Coordinate, from_value: Coordinate = None):
        """
        :param to_value: The destination of the move.
        :param from_value: The origin of the move.
        """
        self.from_value = from_value
        self.to_value = to_value

    def get_delta(self):
        """
        Gets the distance between the origin and the destination.
        :return: The delta of the move as a Vector object.
        """
        return self.to_value.get_distance(self.from_value)

    def reversed(self):
        """
        Reverses the move.
        :return: The reversed move.
        """
        return Move(from_value=self.to_value, to_value=self.from_value)

    def compare_to(self, other: 'Move'):
        """
        Compares two moves.
        :param other: The other move to compare to.
        :return: True if the moves are equal, false otherwise.
        """
        return self.from_value == other.from_value and self.to_value == other.to_value

    def __str__(self) -> str:
        return "Move(from = {}, to = {})".format(self.from_value, self.to_value)


class Team:
    """
    Represents a team in the game.
    """

    def __init__(self, color: str):
        self.one = {
            'opponent': 'TWO',
            'name': 'ONE',
            'letter': 'R',
            'color': 'Rot'
        }
        self.two = {
            'opponent': 'ONE',
            'name': 'TWO',
            'letter': 'B',
            'color': 'Blau'
        }
        self.team_enum = None
        if color == "ONE":
            self.team_enum = self.one
        elif color == "TWO":
            self.team_enum = self.two
        else:
            raise Exception(f"Invalid : {color}")

    def team(self) -> 'Team':
        """
        :return: The team object.
        """
        return self

    def color(self) -> str:
        """
        :return: The color of this team.
        """
        return self.team_enum['name']

    def opponent(self) -> 'Team':
        """
        :return: The opponent of this team.
        """
        return Team(self.team_enum['opponent'])

    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Team) and self.team_enum['name'] == __o.team_enum['name']

    def __str__(self) -> str:
        return f"Team {self.team_enum['name']}."


class Field:
    """
    Represents a field in the game.
    """

    def __init__(self, coordinate: Coordinate, field: int | str | Team):
        self.coordinate = coordinate
        self.field: int | str | Team
        if isinstance(field, int):
            self.field = field
        elif field.isalpha():
            self.field = Team(field)
        else:
            raise TypeError(f"The field's input is wrong: {field}")

    def is_empty(self) -> bool:
        """
        :return: True if the field is has no fishes, False otherwise.
        """
        return self.field == 0

    def is_occupied(self) -> bool:
        """
        :return: True if the field is occupied by a penguin, False otherwise.
        """
        return isinstance(self.field, Team)

    def get_fish(self) -> None | int:
        """
        :return: The amount of fish on the field, None if the field is occupied.
        """
        return None if self.is_occupied() else self.field

    def get_team(self) -> Team | None:
        """
        :return: The team of the field if it is occupied by penguin, None otherwise.
        """
        return self.field if isinstance(self.field, Team) else None

    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, Field) and self.field == __o.field

    def __str__(self):
        return f"This Field is occupied by {self.field}" + (
            " fish(es)." if isinstance(self.field, int) else ".")


class Board:
    """
    Class which represents a game board. Consisting of a two-dimensional array of fields.
    """

    def __init__(self, game_field: list[list[Field]]):
        self.game_field = game_field

    def get_empty_fields(self) -> list[Field]:
        """
        :return: A list of all empty fields.
        """
        fields: list[Field] = []
        for row in self.game_field:
            for field in row:
                if field.is_empty():
                    fields.append(field)
        return fields

    def is_occupied(self, coordinates: Coordinate) -> bool:
        """
        :param coordinates: The coordinates of the field.
        :return: True if the field is occupied, false otherwise.
        """
        return self.get_field(coordinates).is_occupied()

    def is_valid(self, coordinates: Coordinate) -> bool:
        """
        Checks if the coordinates are in the boundaries of the board.
        :param coordinates: The coordinates of the field.
        :return: True if the field is valid, false otherwise.
        """
        arrayCoordinates = coordinates.get_array()
        return 0 <= arrayCoordinates.x < self.width() and 0 <= arrayCoordinates.y < self.height()

    def width(self) -> int:
        """
        :return: The width of the board.
        """
        return len(self.game_field)

    def height(self) -> int:
        """
        :return: The height of the board.
        """
        return len(self.game_field[0])

    def _get_field(self, x: int, y: int) -> Field:
        """
        Gets the field at the given coordinates.
        *Used only internally*

        :param x: The x-coordinate of the field.
        :param y: The y-coordinate of the field.
        :return: The field at the given coordinates.
        """
        return self.game_field[y][x]

    def get_field(self, position: Coordinate) -> Field:
        """
        Gets the field at the given position.
        :param position: The position of the field.
        :return: The field at the given position.
        :raise IndexError: If the position is not valid.
        """
        array_coordinates = position.get_array()
        if self.is_valid(array_coordinates):
            return self._get_field(array_coordinates.x, array_coordinates.y)

        raise IndexError(f"Index out of range: [x={array_coordinates.x}, y={array_coordinates.y}]")

    def get_field_or_none(self, position: Coordinate) -> Field | None:
        """
        Gets the field at the given position no matter if it is valid or not.
        :param position: The position of the field.
        :return: The field at the given position,or None if the position is not valid.
        """
        position = position.get_array()
        if self.is_valid(position):
            return self._get_field(position.x, position.y)
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
        x = index // self.width()
        y = index % self.width()
        return self.get_field(Coordinate(x, y, False))

    def get_all_fields(self) -> list[Field]:
        """
        Gets all hexFields of the board.
        :return: All hexFields of the board.
        """
        return [self.get_field_by_index(i) for i in range(self.width() * self.height())]

    def compare_to(self, other: 'Board') -> list[Field]:
        """
        Compares two boards and returns a list of the hexFields that are different.
        :param other: The other board to compare to.
        :return: A list of hexFields that are different or a empty list if the boards are equal.
        """
        fields = []
        for x in range(len(self.game_field)):
            for y in range(len(self.game_field[0])):
                if self.game_field[x][y] != other.game_field[x][y]:
                    fields.append(self.game_field[x][y])
        return fields

    def contains(self, field: Field) -> bool:
        """
        Checks if the board contains the given field.
        :param field: The field to check for.
        :return: True if the board contains the field, Flase otherwise.
        """
        for row in self.game_field:
            if field in row:
                return True
        return False

    def contains_all(self, fields: list[Field]) -> bool:
        """
        Checks if the board contains all the given fields.
        :param fields: The fields to check for.
        :return: True if the board contains all the given fields, False otherwise.
        """
        for field in fields:
            if not self.contains(field):
                return False
        return True

    def get_moves_in_direction(self, origin: Coordinate, direction: Vector) -> list[Move]:
        """
        Gets all moves in the given direction from the given origin.
        :param origin: The origin of the move.
        :param direction: The direction of the move.
        :return: A list with all moves that fullfill the criteria.
        """
        moves = []
        for i in range(1, self.width()):
            destination = origin.get_double_hex().add_vector(direction.scalar_product(i))
            if self._is_destination_valid(destination):
                moves.append(Move(from_value=origin, to_value=destination))
            else:
                break
        return moves

    def _is_destination_valid(self, field: Coordinate) -> bool:
        return self.is_valid(field) and not self.is_occupied(field) and not \
            self.get_field(field).is_empty()

    def possible_moves_from(self, position: Coordinate) -> list[Move]:
        """
        Returns a list of all possible moves from the given position. That are all moves in all hexagonal directions.
        :param position: The position to start from.
        :return: A list of all possible moves from the given position.
        :raise: IndexError if the position is not valid.
        """
        if not self.is_valid(position):
            raise IndexError(f"Index out of range: [x={position.x}, y={position.y}]")
        moves = []
        for direction in Vector().directions:
            moves.extend(self.get_moves_in_direction(position, direction))
        return moves

    def get_penguins(self) -> list[Field]:
        """
        Searches the board for all penguins.
        :return: A list of all hexFields that are occupied by a penguin.
        """
        return [field for field in self.get_all_fields() if field.is_occupied()]

    def get_teams_penguins(self, team: Team) -> list[Coordinate]:
        """
        Searches the board for all penguins of the given team.
        :param team: The team to search for.
        :return: A list of all coordinates that are occupied by a penguin of the given team.
        """
        teams_penguins = []
        for x in range(self.width()):
            for y in range(self.height()):
                current_field = self.get_field(Coordinate(x, y, False))
                if current_field.is_occupied() and current_field.get_team().team() == team:
                    coordinates = Coordinate(x, y, False).get_double_hex()
                    teams_penguins.append(coordinates)
        return teams_penguins

    def get_most_fish(self) -> list[Field]:
        """
        Returns a list of all fields with the most fish.
        :return: A list of Fields.
        """
        fields = self.get_all_fields()
        fields.sort(key=lambda field: field.get_fish(), reverse=True)
        for i, field in enumerate(fields):
            if field.get_fish() < fields[0].get_fish():
                fields = fields[:i]
        return fields

    def get_board_intersection(self, other: 'Board') -> list[Field]:
        """
        Returns a list of all fields that are in both boards.
        :param other: The other board to compare to.
        :return: A list of Fields.
        """
        return [field for field in self.get_all_fields() if field in other.get_all_fields()]

    def get_fields_intersection(self, other: list[Field]) -> list[Field]:
        """
        Returns a list of all fields that are in both list of Fields.
        :param other: The other list of Fields to compare to.
        :return: A list of Fields.
        """
        return [field for field in self.get_all_fields() if field in other]

    @staticmethod
    def get_field_intersection(first: list[Field], second: list[Field]) -> list[Field]:
        """
        Returns a list of all fields that are in both list of Fields.
        :param first: The first list of Fields to compare to.
        :param second: The second list of Fields to compare to.
        :return: A list of Fields.
        """
        return [field for field in first if field in second]

    @staticmethod
    def get_move_intersection(first: list[Move], second: list[Move]) -> list[Move]:
        """
        Returns a list of all moves that are in both list of Fields.
        :param first: The first list of moves to compare to.
        :param second: The second list of moves to compare to.
        :return: A list of moves.
        """
        return [move for move in first if move in second]

    @staticmethod
    def get_move_field_intersection(moves: list[Move], fields: list[Field]) -> list[Move]:
        """
        Returns a list of all moves that to-coordinates are the coordinates of a field in the list of fields.
        :param moves: The list of moves that coordinates to compare to.
        :param fields: The list of fields that coordinates to compare to.
        :return: A list of moves.
        """
        intersection = []
        for move in moves:
            for field in fields:
                if move.to_value == field.coordinate:
                    intersection.append(move)
        return intersection

    def __eq__(self, __o: 'Board'):
        return self.game_field == __o.game_field


class Fishes:
    """
    Represents the amount of fish each player has.
    """

    def __init__(self, fishes_one: int, fishes_two: int):
        self.fishes_one = fishes_one
        self.fishes_two = fishes_two

    def get_fish_by_team(self, team: Team):
        """
        Looks up the amount of fish a team has.
        :param team: A team object, that represents the team to get the fish amount of.
        :return: The amount of fish of the given team.
        """
        return self.fishes_one if team.team_enum == Team("ONE").team_enum else self.fishes_two


class GameState:
    """
       A `GameState` contains all information, that describes the game state at a given time, that is, between two game
       moves.

       This includes:
         - a consecutive turn number (round & turn) and who's turn it is
         - the board
         - the last move made

       The `GameState` is thus the central object through which all essential information of the current game can be
       accessed.

       Therefore, for easier handling, it offers further aids, such as:
         - a method to calculate available moves and to execute moves

       The game server sends a new copy of the `GameState` to both participating players after each completed move,
       describing the then current state. Information about the course of the game can only be obtained from the
       `GameState` to a limited extent and must therefore be recorded by a game client itself if necessary.

       In addition to the actual information certain partial information can be queried.
       """

    def __init__(self, board: Board, turn: int, start_team: Team, fishes: Fishes, last_move: Move = None):
        """
        Creates a new `GameState` with the given parameters.
        :param board: The board of the game.
        :param turn: The turn number of the game.
        :param start_team: The team that has the first turn.
        :param fishes: The number of fishes each team has.
        :param last_move: The last move made.
        """
        self.start_team = start_team
        self.board = board
        self.turn = turn
        self.round = int((self.turn + 1) / 2)
        self.current_team = self.current_team_from_turn()
        self.other_team = self.current_team_from_turn().opponent()
        self.last_move = last_move
        self.fishes = fishes
        self.current_pieces = self.board.get_teams_penguins(self.current_team)

    def get_possible_moves(self, current_team: Team = None) -> list[Move]:
        """
        Gets all possible moves for the current team.
        That includes all possible moves from all hexFields that are not occupied by a penguin from that team.
        :return: A list of all possible moves from the current player's turn.
        """
        current_team = current_team or self.current_team
        moves = []
        if len(self.board.get_teams_penguins(current_team)) < 4:
            for x in range(self.board.width() - 1):
                for y in range(self.board.height() - 1):
                    field = self.board.get_field(Coordinate(x, y, False))
                    if not field.is_occupied() and field.get_fish() == 1:
                        moves.append(Move(from_value=None, to_value=Coordinate(x, y, False).get_double_hex()))
        else:
            for piece in self.board.get_teams_penguins(current_team):
                moves.extend(self.board.possible_moves_from(piece))
        return moves

    def get_most_fish_moves(self) -> list[Move]:
        """
        Returns a list of all Moves that will get the most fish from possible moves.
        :return: A list of Moves.
        """
        moves = self.get_possible_moves()
        moves.sort(key=lambda move: self.board.get_field(move.to_value).get_fish(), reverse=True)
        for i, move in enumerate(moves):
            first_fish = self.board.get_field(moves[0].to_value).get_fish()
            current_fish = self.board.get_field(move.to_value).get_fish()
            if first_fish and current_fish:
                if current_fish < first_fish:
                    moves = moves[:i]
                    break
        return moves

    def current_team_from_turn(self) -> Team:
        """
        Calculates the current team from the turn number.
        :return: The team that has the current turn.
        """
        current_team_by_turn = self.start_team if self.turn % 2 == 0 else self.start_team.opponent()
        if not self.get_possible_moves(current_team_by_turn):
            return current_team_by_turn.opponent()
        return current_team_by_turn
