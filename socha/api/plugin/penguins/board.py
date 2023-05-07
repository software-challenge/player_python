import _pickle as pickle
import dataclasses
import logging
from dataclasses import dataclass
from itertools import chain, takewhile
from operator import attrgetter
from typing import List, Union, Optional, Generator, Iterator

from socha.api.plugin.penguins.coordinate import HexCoordinate, Vector, CartesianCoordinate
from socha.api.plugin.penguins.team import Penguin, TeamEnum, Move


@dataclass(frozen=True, order=True, unsafe_hash=True)
class Field:
    """
    Represents a field in the game.
    """

    coordinate: HexCoordinate
    penguin: Optional[Penguin]
    fish: int

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


@dataclass(frozen=True, order=True, unsafe_hash=True)
class Board:
    """
    Class which represents a game board. Consisting of a two-dimensional array of fields.
    """

    board: List[List[Field]]

    def get_empty_fields(self) -> List[Field]:
        """
        :return: A list of all empty fields.
        """
        return list(filter(lambda field: field.is_empty(), chain(*self.board)))

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
        array_coordinates = coordinates.to_cartesian()
        return 0 <= array_coordinates.x < self.width() and 0 <= array_coordinates.y < self.height()

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
        return self._get_field(cartesian.x, cartesian.y) if self.is_valid(position) else None

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
        :return: A list of Fields that are different or an empty list if the boards are equal.
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
        return any(field in row for row in self.board)

    def contains_all(self, fields: List[Field]) -> bool:
        """
        Checks if the board contains all the given fields.

        :param fields: The fields to check for.
        :return: True if the board contains all the given fields, False otherwise.
        """
        if not fields:
            return False

        return all(self.contains(field) for field in fields)

    def get_moves_in_direction(self, origin: HexCoordinate, direction: Vector, team_enum: Optional[TeamEnum] = None) \
            -> List[Move]:
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
        team_enum = team_enum or self.get_field(origin).penguin.team_enum
        if not self.get_field(origin).penguin or self.get_field(origin).penguin.team_enum != team_enum:
            return []

        def valid_destination(i):
            destination = origin.add_vector(direction.scalar_product(i))
            return self.is_destination_valid(destination)

        moves = [Move(team_enum=team_enum, from_value=origin, to_value=origin.add_vector(direction.scalar_product(i)))
                 for i in takewhile(valid_destination, range(1, self.width()))]

        return moves

    def is_destination_valid(self, field: HexCoordinate) -> bool:
        """
        Checks if the given field is a valid destination for a move.
        It checks if the destination is on the board, if it is not occupied and if it is not empty.

        Args:
            field: The field to check for.

        Returns:
            bool: True if the field is a valid destination, False otherwise.
        """
        return self.is_valid(field) and not self.is_occupied(field) and not \
            self.get_field(field).is_empty()

    def _is_destination_valid(self, field: HexCoordinate) -> bool:
        return self.is_destination_valid(field)

    def possible_moves_from(self, position: HexCoordinate, team_enum: Optional[TeamEnum] = None) -> List[Move]:
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
        if not self.get_field(position).penguin or (
                team_enum and self.get_field(position).penguin.team_enum != team_enum):
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
        penguins = filter(lambda field: field.penguin and field.penguin.team_enum == team,
                          (field for row in self.board for field in row))
        return list(map(attrgetter('penguin'), penguins))

    def get_most_fish(self) -> List[Field]:
        """
        Returns a list of all fields with the most fish.

        :return: A list of Fields.
        """
        fields = [field for field in self.get_all_fields() if not field.is_occupied()]
        max_fish = max(fields, key=lambda field: field.get_fish()).get_fish()
        return list(filter(lambda field: field.get_fish() == max_fish, fields))

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

    def get_neighbor_fields(self, field: Field) -> Iterator[Field]:
        """
        Returns a generator of all neighbor fields of the given field.

        Args:
            field: The field to get the neighbors of.

        Returns:
            Generator[Field, None, None]: A generator of all neighbor fields of the given field.
        """
        return (self.get_field(each) for each in field.coordinate.get_neighbors() if self.is_valid(each))

    def get_neighbor_fields_coordinate(self, coordinate: HexCoordinate) -> Iterator[Field]:
        """
        Returns a generator of all neighbor fields of the given coordinate.

        Args:
            coordinate: The coordinate to get the neighbors of.

        Returns:
            Generator[Field, None, None]: A generator of all neighbor fields of the given coordinate.
        """
        return (self.get_field(each) for each in coordinate.get_neighbors() if self.is_valid(each))

    def get_valid_neighbor_fields(self, field: Field) -> Iterator[Field]:
        """
        Returns a generator of all neighbor fields of the given field.

        Args:
            field: The field to get the neighbors of.

        Returns:
            Generator[Field, None, None]: A generator of all neighbor fields of the given field.
        """
        return (self.get_field(each) for each in field.coordinate.get_neighbors() if self.is_destination_valid(each))

    def get_valid_neighbor_fields_coordinate(self, coordinate: HexCoordinate) -> Iterator[Field]:
        """
        Returns a generator of all neighbor fields of the given coordinate.

        Args:
            coordinate: The coordinate to get the neighbors of.

        Returns:
            Generator[Field, None, None]: A generator of all neighbor fields of the given coordinate.
        """
        return (self.get_field(each) for each in coordinate.get_neighbors() if self.is_destination_valid(each))

    def move(self, move: Move) -> 'Board':
        """
        Moves the penguin from the origin to the destination.
        **Please make sure that the move is correct, because this method will not check that.**
        If there is no Penguin to move, than this method will return the current state unchanged.

        :param move: The move to execute.
        :return: The new board with the moved penguin.
        """
        board_state = pickle.loads(pickle.dumps(self.board, protocol=-1))
        moving_penguin = Penguin(team_enum=move.team_enum, coordinate=move.to_value)
        if move.from_value:
            if not self.get_field(move.from_value).penguin:
                logging.error(f"There is no penguin to move. Origin was: {self.get_field(move.from_value)}")
                return self
            origin_field_coordinate = move.from_value.to_cartesian()
            moving_penguin = board_state[origin_field_coordinate.y][origin_field_coordinate.x].penguin
            moving_penguin = dataclasses.replace(moving_penguin, coordinate=move.to_value)
            board_state[origin_field_coordinate.y][origin_field_coordinate.x] = Field(coordinate=move.from_value,
                                                                                      penguin=None, fish=0)

        destination_field = Field(coordinate=move.to_value, penguin=moving_penguin, fish=0)
        board_state[move.to_value.to_cartesian().y][move.to_value.to_cartesian().x] = destination_field
        return Board(board_state)

    def pretty_print(self):
        print()
        for i, row in enumerate(self.board):
            row_str = ""
            if (i + 1) % 2 == 0:
                row_str += " "
            row_str += " ".join(["~" if field.is_empty() else field.get_team().value[0] if field.is_occupied() else str(
                field.get_fish())
                                 for field in row])
            print(row_str)
        print()
