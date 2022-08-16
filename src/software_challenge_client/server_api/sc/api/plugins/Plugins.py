import src.software_challenge_client.server_api.xflux.XFluxDecorator as XStrDec
from src.software_challenge_client.server_api.sc.api.plugins.IPlugins import IMove


class Vector:
    """
    Represents a vector in the hexagonal grid. It can calculate various vector operations.
    """

    def __init__(self, dx: int, dy: int):
        """
        Constructor for the Vector class.
        :param dx: The x-coordinate of the vector.
        :param dy: The y-coordinate of the vector.
        """
        self.dx = dx
        self.dy = dy

    def length(self):
        """
        Calculates the length of the vector.
        :return: The length of the vector.
        """
        return (self.dx ** 2 + self.dy ** 2) ** 0.5

    def times(self, scalar: int):
        """
        Extends the vector by a scalar.
        :param scalar: The scalar to extend the vector by.
        :return: The extended vector.
        """
        return Vector(self.dx * scalar, self.dy * scalar)

    def plus(self, other: 'Vector'):
        """
        Adds two vectors.
        :param other: The other vector to add.
        :return: The sum of the two vectors as a new vector object.
        """
        return Vector(self.dx + other.dx, self.dy + other.dy)

    def minus(self, other: 'Vector'):
        """
        Subtracts two vectors.
        :param other: The other vector to subtract.
        :return: The difference of the two vectors as a new vector object.
        """
        return Vector(self.dx - other.dx, self.dy - other.dy)

    def compareTo(self, other: 'Vector'):
        """
        Compares two vectors.
        :param other: The other vector to compare to.
        :return: True if the vectors are equal, false otherwise.
        """
        return self.dx == other.dx and self.dy == other.dy

    def getHexNeighbors(self):
        """
        Gets the six neighbors of the vector.
        :return: A list of the six neighbors of the vector.
        """
        return [
            self.plus(Vector(1, -1)),  # UP RIGHT
            self.plus(Vector(-2, 0)),  # RIGHT
            self.plus(Vector(1, 1)),  # DOWN RIGHT
            self.plus(Vector(-1, 1)),  # DOWN LEFT
            self.plus(Vector(2, 0)),  # LEFT
            self.plus(Vector(-1, -1))  # UP LEFT
        ]

    def isOneHexMove(self):
        """
        Checks if the vector is a one hex move.
        :return: True if the vector is a one hex move, false otherwise.
        """
        return abs(self.dx) == abs(self.dy) or (self.dx % 2 == 0 and self.dy == 0)


@XStrDec.alias(name='coordinates')
class Coordinates:
    """
    Representation of a coordination system in the hexagonal grid.
    """

    def __init__(self, x: int, y: int):
        """
        Constructor for the Coordinates class.
        :param x: The x-coordinate of the coordination system.
        :param y: The y-coordinate of the coordination system.
        """
        self.x = x
        self.y = y

    def __str__(self) -> str:
        return "[{}, {}]".format(self.x, self.y)

    def addVector(self, vector: Vector):
        """
        Adds a vector to the coordinate.
        :param vector: The vector to add.
        :return: The new coordinate.
        """
        return Coordinates(self.x + vector.dx, self.y + vector.dy)

    def minusVector(self, vector: Vector):
        """
        Subtracts a vector from the coordinate.
        :param vector: The vector to subtract.
        :return: The new coordinate.
        """
        return Coordinates(self.x - vector.dx, self.y - vector.dy)

    def getDistance(self, other: 'Coordinates'):
        """
        Calculates the distance between two coordinates.
        :param other: The other coordinate to calculate the distance to.
        :return: The distance between the two coordinates as Vector object.
        """
        return Vector(self.x - other.x, self.y - other.y)

    def getVector(self):
        """
        Gets the vector from the coordinate to the origin.
        :return: The vector from the coordinate to the origin.
        """
        return Vector(self.x, self.y)

    def getHexNeighbors(self):
        """
        Gets the six neighbors of the coordinate.
        :return: A list of the six neighbors of the coordinate.
        """
        return self.getVector().getHexNeighbors()


@XStrDec.alias(name='move')
class Move(IMove):
    """
    Represents a move in the game. 
    """

    def __init__(self, toCoo: Coordinates, fromCoo: Coordinates = None):
        """
        :param toCoo: The destination of the move.
        :param fromCoo: The origin of the move.
        """
        self.__from = fromCoo
        self.__to = toCoo

    def getDelta(self):
        """
        Gets the distance between the origin and the destination.
        :return: The delta of the move as a Vector object.
        """
        return self.__to.getDistance(self.__from)

    def reversed(self):
        """
        Reverses the move.
        :return: The reversed move.
        """
        return Move(self.__from, self.__to)

    def compareTo(self, other: 'Move'):
        """
        Compares two moves.
        :param other: The other move to compare to.
        :return: True if the moves are equal, false otherwise.
        """
        return self.__from.compareTo(other.__from) and self.__to.compareTo(other.__to)

    def __str__(self) -> str:
        return "Move from {} to {}".format(self.__from, self.__to)

    def move(self, origin: Coordinates, delta: Vector) -> 'Move':
        """
        Executes the move to the destination.
        :param origin: The origin of the move.
        :param delta: The delta of the move.
        :return: The new move.
        """
        return Move(origin.addVector(delta), origin)

    def setDestination(self, destination: Coordinates):
        """
        Sets the new destination of the move.
        :param destination: The new destination of the move.
        """
        self.__to = destination
