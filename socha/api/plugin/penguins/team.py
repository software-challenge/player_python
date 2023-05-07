from dataclasses import dataclass
from enum import Enum
from typing import List, Optional

from socha.api.plugin.penguins.coordinate import HexCoordinate, Vector


class TeamEnum(Enum):
    ONE = "ONE"
    TWO = "TWO"


@dataclass(frozen=True, order=True, unsafe_hash=True)
class Move:
    """
    Represents a move in the game.
    """

    team_enum: TeamEnum
    from_value: Optional[HexCoordinate]
    to_value: HexCoordinate

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


@dataclass(frozen=True, order=True, unsafe_hash=True)
class Penguin:
    """
       The Penguin class represents a penguin object with a coordinate and a team_enum.
    """

    coordinate: HexCoordinate
    team_enum: TeamEnum

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


@dataclass(order=True, unsafe_hash=True)
class Team:
    """
    The Team class is useful for storing and manipulating information about teams in the game. It allows you to
    easily create objects for each team_enum, keep track of their attributes, and compare them to their opponents.
    """

    name: TeamEnum
    fish: int
    penguins: List[Penguin]
    moves: List[Move]
    opponent: Optional['Team'] = None

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
        return f'Team(name={self.name.value}, fish={self.fish}, penguins={len(self.penguins)}, ' \
               f'moves={len(self.moves)}, opponent={None if not self.opponent else self.opponent.name})'
