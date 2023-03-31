from enum import Enum
from typing import List, Optional

from socha.api.plugin.penguins.coordinate import HexCoordinate, Vector


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


class Team:
    """
    The Team class is useful for storing and manipulating information about teams in the game. It allows you to
    easily create objects for each team_enum, keep track of their attributes, and compare them to their opponents.
    """

    def __init__(self, name: TeamEnum, fish: int, penguins: List[Penguin], moves: List[Move],
                 opponent: Optional['Team'] = None):
        self.name = name
        self.fish = fish
        self.penguins = penguins
        self.moves = moves
        self.opponent = opponent

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

    def __repr__(self) -> str:
        return f"Team(name={self.name}, fish={self.fish}, penguins={len(self.penguins)}, moves={len(self.moves)})"

    def __eq__(self, other):
        if isinstance(other, self.__class__):
            return self.name == other.name and self.fish == other.fish and self.penguins == other.penguins and \
                   self.moves == other.moves
        return False
