import math
from dataclasses import dataclass
from typing import List, Optional


@dataclass(frozen=True, order=True, unsafe_hash=True)
class Vector:
    """
    Represents a vector in the hexagonal grid. It can calculate various vector operations.
    """
    d_x: int = 0
    d_y: int = 0

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


@dataclass(frozen=True, order=True, unsafe_hash=True)
class Coordinate:
    x: int
    y: int

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


@dataclass(frozen=True, order=True, unsafe_hash=True)
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


@dataclass(frozen=True, order=True, unsafe_hash=True)
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
