import src.SoftwareChallengeClient.server_api.networking.xflux.XFluxDecorator as XStrDec
from src.SoftwareChallengeClient.server_api.networking.xflux.XFluxInterface import ImplicitArray, Attribute, Traverse
from src.SoftwareChallengeClient.server_api.sc.api.plugins.IPlugins import IMove, IBoard, IField, ITeam, IGameState


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

    def toCoordinates(self) -> 'Coordinates':
        """
        Converts the vector to coordinate object.
        :return:    The coordinate object.
        """
        return Coordinates(self.dx, self.dy)


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

    def addVector(self, vector: Vector) -> 'Coordinates':
        """
        Adds a vector to the coordinate.
        :param vector: The vector to add.
        :return: The new coordinate.
        """

        return self.getVector().plus(vector).toCoordinates()

    def minusVector(self, vector: Vector) -> 'Coordinates':
        """
        Subtracts a vector from the coordinate.
        :param vector: The vector to subtract.
        :return: The new coordinate.
        """
        return self.getVector().minus(vector).toCoordinates()

    def getDistance(self, other: 'Coordinates') -> 'Coordinates':
        """
        Calculates the distance between two coordinates.
        :param other: The other coordinate to calculate the distance to.
        :return: The distance between the two coordinates as Vector object.
        """
        return self.getVector().minus(other.getVector()).toCoordinates()

    def getVector(self) -> Vector:
        """
        Gets the vector from the coordinate to the origin.
        :return: The vector from the coordinate to the origin.
        """
        return Vector(self.x, self.y)

    def getHexNeighbors(self) -> list[Vector]:
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


"""
=====================================================================================================================
=====================================================================================================================
"""


@XStrDec.alias(name='field')
class Team(ITeam):
    ONE = {}
    TWO = {}

    def __init__(self, index: int):
        self.ONE = {
            'opponent': self.TWO,
            'index': 1,
            'letter': 'R',
            'color': 'Rot'
        }
        self.TWO = {
            'opponent': self.ONE,
            'index': 2,
            'letter': 'B',
            'color': 'Blau'
        }
        self.teamEnum = None
        if index is 0:
            self.teamEnum = self.ONE
        else:
            self.teamEnum = self.TWO

    def color(self) -> str:
        return self.teamEnum['color']

    def opponent(self) -> 'Team':
        return Team(self.teamEnum['opponent']['index'])


@XStrDec.alias(name='field')
class Field(IField):
    def __init__(self, fish: int = 0, penguin: Team = None):
        self.fish = fish
        self.penguin = penguin
        super().__init__()

    def isEmpty(self) -> bool:
        return self.fish == 0 and self.penguin is None

    def isOccupied(self) -> bool:
        return self.penguin is not None

    def __copy__(self):
        return Field(self.fish, self.penguin)

    def __str__(self):
        return "Field with {} fish and {} penguin".format(self.fish, self.penguin)


class HexBoard(IBoard):
    def __init__(self, gameField: list[list[Field]] = None):
        self.__gameField = ImplicitArray(caller=self, fieldName="gameField", fieldValue=gameField)

    def areFieldsEmpty(self) -> bool:
        for row in self.__gameField.fieldValue:
            for field in row:
                if not field.isEmpty():
                    return False
        return True

    def isOccupied(self, coordinates: Coordinates) -> bool:
        return self.__gameField.fieldValue[coordinates.x][coordinates.y].isOccupied()

    def isValid(self, coordinates: Coordinates) -> bool:
        return 0 <= coordinates.x < len(self.__gameField.fieldValue) and 0 <= coordinates.y < len(self.__gameField.
                                                                                                  fieldValue[0])

    def width(self) -> int:
        return len(self.__gameField.fieldValue)

    def height(self) -> int:
        return len(self.__gameField.fieldValue[0])

    def __getField(self, x: int, y: int) -> Field:
        """
        Gets the field at the given coordinates.
        *Used only internally*

        :param x: The x-coordinate of the field.
        :param y: The y-coordinate of the field.
        :return: The field at the given coordinates.
        """
        return self.__gameField.fieldValue[x][y]

    def getField(self, position: Coordinates) -> Field:
        """
        Gets the field at the given position.
        :param position: The position of the field.
        :return: The field at the given position.
        :raise IndexError: If the position is not valid.
        """
        if self.isValid(position):
            return self.__getField(position.x, position.y)
        else:
            raise IndexError("Index out of range")

    def getFieldOrNone(self, position: Coordinates) -> Field | None:
        """
        Gets the field at the given position no matter if it is valid or not.
        :param position: The position of the field.
        :return: The field at the given position,or None if the position is not valid.
        """
        if self.isValid(position):
            return self.__getField(position.x, position.y)
        else:
            return None

    def getFieldByIndex(self, index: int) -> Field:
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
        return self.getField(Coordinates(x, y))

    def getAllFields(self) -> list[Field]:
        """
        Gets all fields of the board.
        :return: All fields of the board.
        """
        return [self.getFieldByIndex(i) for i in range(self.width() * self.height())]

    def compareTo(self, other: 'HexBoard') -> list[Field]:
        """
        Compares two boards and returns a list of the fields that are different.
        :param other: The other board to compare to.
        :return: A list of fields that are different or a empty list if the boards are equal.
        """
        fields = []
        for x in range(len(self.__gameField.fieldValue)):
            for y in range(len(self.__gameField.fieldValue[0])):
                if self.__gameField.fieldValue[x][y] != other.__gameField.fieldValue[x][y]:
                    fields.append(self.__gameField.fieldValue[x][y])
        return fields

    def contains(self, field: Field) -> bool:
        for row in self.__gameField.fieldValue:
            if field in row:
                return True
        return False

    def containsAll(self, fields: list[Field]) -> bool:
        for field in fields:
            if not self.contains(field):
                return False
        return True

    def __str__(self) -> str:
        return '\n'.join([' '.join([str(field) for field in row]) for row in self.__gameField.fieldValue])

    def __copy__(self):
        return HexBoard(self.__gameField.fieldValue)

    def __eq__(self, other):
        return self.compareTo(other)

    def __hash__(self) -> int:
        return hash(self.__gameField.fieldValue)


@XStrDec.alias(name='board')
class Board(IBoard, HexBoard):
    """
    Class which represents a game board. Consisting of a two-dimensional array of fields.
    """

    def __init__(self, fields: HexBoard):
        self.__fields = Traverse(self, fields)

    @property
    def fields(self) -> HexBoard:
        return self.__fields.fieldValue

    def setPenguin(self, position: Coordinates, team: Team) -> int:
        """
        Sets the penguin at the given position and removes the fish from the field.
        :param position: The position of the penguin.
        :param team: The team of the penguin.
        :raise IndexError: If the position is not valid.
        :return: The number of fish that were removed from the field.
        """
        if self.fields.isValid(position):
            field = self.fields.getField(position)
            if field.isOccupied():
                raise Exception("Field is already occupied")
            else:
                field.penguin = team
                return field.fish
        else:
            raise IndexError("Index out of range")

    def possibleMovesFrom(self, position: Coordinates) -> list[Move]:
        """
        Returns a list of all possible moves from the given position.
        :param position: The position to start from.
        :return: A list of all possible moves from the given position.
        """
        if self.fields.isValid(position):
            neighbours = position.getNeighbours()
            moves = []
            for neighborVector in neighbours:
                neighborCoordinate = position.addVector(neighborVector)
                if self.fields.isValid(neighborCoordinate):
                    move = Move(position, neighborCoordinate)
                    if not self.fields.getField(neighborCoordinate).isOccupied():
                        moves.append(move)
                else:
                    raise IndexError("Index out of range")
            return moves
        else:
            raise IndexError("Index out of range")

    def getPenguins(self) -> list[Field]:
        """
        Searches the board for all penguins.
        :return: A list of all fields that are occupied by a penguin.
        """
        return [field for field in self.fields.getAllFields() if field.isOccupied()]

    def getTeamsPenguins(self, team: ITeam) -> list[Field]:
        """
        Searches the board for all penguins of the given team.
        :param team: The team to search for.
        :return: A list of all fields that are occupied by a penguin of the given team.
        """
        return [field for field in self.fields.getAllFields() if field.isOccupied() and field.penguin == team]

    def __eq__(self, other):
        return self.fields == other.fields


class TwoPlayerGameState(IGameState):
    def __init__(self, startTeam: Team):
        self.startTeam = startTeam
        self.round = int((self.turn + 1) / 2)
        self.currentTeam = self.currentTeamFromTurn()
        self.otherTeam = self.currentTeamFromTurn().opponent()
        self.lastMove = None

    def performMove(self, move: Move):
        ...

    def currentTeamFromTurn(self) -> Team:
        return self.startTeam if self.turn % 2 == 0 else self.startTeam.other()

    def __str__(self):
        return "GameState[turn={}, currentTeam={}, lastMove={}]".format(self.turn, self.currentTeam, self.lastMove)


@XStrDec.alias(name='state')
class GameState(TwoPlayerGameState):
    def __init__(self, board: Board, turn: int = 0, lastMove: Move = None, fishes: list = None):
        super().__init__(Team(0))
        self.__board = Traverse(self, board)
        self.__turn = Attribute(caller=self, fieldName="turn", fieldValue=turn)
        self.lastMove = lastMove
        self.fishes = fishes
        self.currentPieces = self.board.getTeamsPenguins(self.currentTeam)

    @property
    def board(self) -> Board:
        return self.__board.fieldValue

    def getPossibleMoves(self) -> list[Move]:
        moves = []
        for piece in self.currentPieces:
            moves.extend(self.board.possibleMovesFrom(piece.coordinates))
        return moves

    def __eq__(self, __o: object) -> bool:
        return isinstance(__o, GameState) and self.board == __o.board and self.turn == __o.turn and self.lastMove == \
               __o.lastMove and self.fishes == __o.fishes and self.currentPieces == __o.currentPieces
