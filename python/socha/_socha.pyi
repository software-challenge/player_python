from enum import Enum

class Vector:
    """
    Ein 2 dimensionaler Vektor.

    Attributes:
        delta_x (int): Die Entfernung in x-Richtung.
        delta_y (int): Die Entfernung in y-Richtung.
    """

    delta_x: int
    delta_y: int

    def __init__(self, delta_x: int, delta_y: int) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    def deepcopy(self) -> Vector:
        """Kopiert das Objekt."""

    def add_vector(self, other: Vector) -> Vector:
        """Addiert einen anderen Vector (nicht mutierend)."""

    def add_vector_mut(self, other: Vector) -> None:
        """Addiert einen anderen Vector (mutierend)."""

    def scale(self, scalar: int) -> Vector:
        """Skaliert diesen Vektor (nicht mutierend)."""

    def scale_mut(self, scalar: int) -> None:
        """Skaliert diesen Vektor (mutierend)."""

    def get_length(self) -> float | None:
        """Berechnet die Länge dieses Vektors."""


class Coordinate:
    """
    Eine 2 dimensionale Koordinate auf einem Spielfeld.

    Attributes:
        x (int): Der x-Wert.
        y (int): Der y-Wert.
    """

    x: int
    y: int

    def __init__(self, x: int, y: int) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    def deepcopy(self) -> Coordinate:
        """Kopiert das Objekt."""

    def add_vector(self, vector: Vector) -> Coordinate:
        """Addiert einen Vector auf diese Koordinate (nicht mutierend)."""

    def add_vector_mut(self, vector: Vector) -> None:
        """Addiert einen Vector auf diese Koordinate (mutierend)."""

    def get_difference(self, other: Coordinate) -> Vector:
        """Berechnet die Differenz zwischen zwei Koordinaten als Vektor."""

    def neighbors(self) -> list[Coordinate]:
        """Gibt die vier benachbarten Feldkoordinaten zurück."""

    def diagonal_neighbors(self) -> list[Coordinate]:
        """Gibt die vier angrenzenden Ecken der Feldkoordinaten zurück."""

    def as_vector(self) -> Vector:
        """Coordinate als Vektor Objekt"""


class Direction(Enum):
    """Eine Darstellung für eine normierte Richtung."""

    Up = 0
    UpRight = 1
    Right = 2
    DownRight = 3
    Down = 4
    DownLeft = 5
    Left = 6
    UpLeft = 7

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def deepcopy(self) -> Direction: ...

    @staticmethod
    def from_vector(vector: Vector) -> Direction | None:
        """Wandelt einen Vektor in eine der 8 Richtungen um."""

    @staticmethod
    def all_directions() -> list[Direction]:
        """Gibt eine Liste aller 8 Richtungen zurück."""

    @staticmethod
    def cardinals() -> list[Direction]:
        """Gibt die vier nicht-diagonalen Richtungen zurück (Up, Right, Down, Left)."""

    @staticmethod
    def diagonals() -> list[Direction]:
        """Gibt die vier diagonalen Richtungen zurück."""

    def to_vector(self) -> Vector:
        """Wandelt die Richtung in den entsprechenden Vektor um."""

    def to_mirrored(self) -> Direction:
        """Spiegelt die gegebene Richtung."""


class TeamEnum(Enum):
    """Eine Darstellung für die beiden Teams."""

    One = 0
    Two = 1

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    def opponent(self) -> TeamEnum:
        """Gibt den Gegner dieses Teams zurück."""


class Color(Enum):
    """Die Farbe eines Spielsteins / Teams."""

    BLUE = 0
    YELLOW = 1
    RED = 2
    GREEN = 3

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

    def next(self) -> Color:
        """Gibt die nächste Farbe in der Zugreihenfolge zurück."""

    def team(self) -> TeamEnum:
        """Gibt das Team zurück, zu dem diese Farbe gehört."""

    def to_field_content(self) -> FieldContent:
        """Wandelt die Farbe in den entsprechenden Feldinhalt um."""

    def name(self) -> str:
        ...


class FieldContent(Enum):
    """Der Inhalt eines Feldes: eine Farbe oder leer."""

    BLUE = 0
    YELLOW = 1
    RED = 2
    GREEN = 3
    EMPTY = 4

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    def to_team_color(self) -> Color | None:
        """Wandelt den Feldinhalt in die entsprechende Farbe um, oder None, wenn leer."""

    def is_empty(self) -> bool:
        """Gibt zurück, ob das Feld leer ist."""


class Field:
    """Ein einzelnes Feld auf dem Spielbrett."""

    coordinate: Coordinate
    content: FieldContent

    def __init__(self, coordinate: Coordinate, color: Color) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    def deepcopy(self) -> Field:
        """Kopiert das Objekt."""

    def is_empty(self) -> bool:
        """Gibt zurück, ob das Feld leer ist."""


class Board:
    """
    Das Spielbrett.

    Attributes:
        map (list[list[Field]]): Die 2-dimensionale Liste der Felder.
    """

    map: list[list[Field]]

    def __init__(self, map: list[list[Field]] | None) -> None: ...
    def __eq__(self, other: object) -> bool: ...

    def get(self, position: Coordinate) -> Field:
        """
        Gibt das Feld an der gegebenen Koordinate zurück.

        Raises:
            IndexError: Wenn die Koordinate außerhalb des Spielfelds liegt.
        """

    def set_content(self, position: Coordinate, content: FieldContent) -> None:
        """Setzt den Inhalt eines Feldes."""

    def get_content(self, position: Coordinate) -> FieldContent | None:
        """Gibt den Inhalt eines Feldes zurück, oder None, wenn außerhalb des Feldes."""

    def is_empty(self) -> bool:
        """Prüft, ob alle Felder leer sind."""

    def is_obstructed(self, position: Coordinate) -> bool:
        """Prüft, ob auf dieser Position bereits eine Spielerfarbe liegt."""

    def get_team(self, position: Coordinate) -> Color | None:
        """Gibt das Team zurück, das auf dem Feld liegt, oder None."""

    def pretty_string(self) -> str:
        """Gibt eine lesbare String-Darstellung des Spielfelds zurück."""

    def compare(self, other: Board) -> list[Field]:
        """Vergleicht dieses Board mit einem anderen und gibt die unterschiedlichen Felder zurück."""

    @staticmethod
    def random_fields() -> list[list[Field]]:
        """Erstellt ein leeres Spielfeld."""

    @staticmethod
    def contains(position: Coordinate) -> bool:
        """Prüft, ob die Koordinate innerhalb der Grenzen des Spielfelds liegt."""


class Rotation(Enum):
    """Beschreibt, wie weit eine PieceShape gedreht werden soll."""

    NONE = 0
    RIGHT = 1
    MIRROR = 2
    LEFT = 3

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    def value(self) -> int:
        """Gibt den numerischen Wert (Anzahl Vierteldrehungen) zurück."""

    def rotate(self, other: Rotation) -> Rotation:
        """Summiert beide Rotationen auf."""

    @staticmethod
    def all() -> list[Rotation]:
        """Gibt alle vier Rotationen zurück."""

    def name(self) -> str:
        ...

class PieceShape(Enum):
    """Eine Enumeration aller 21 verschiedenen Formen."""

    Mono = 0
    Domino = 1
    TrioL = 2
    TrioI = 3
    TetroO = 4
    TetroT = 5
    TetroI = 6
    TetroL = 7
    TetroZ = 8
    PentoL = 9
    PentoT = 10
    PentoV = 11
    PentoS = 12
    PentoZ = 13
    PentoI = 14
    PentoP = 15
    PentoW = 16
    PentoU = 17
    PentoR = 18
    PentoX = 19
    PentoY = 20

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    @staticmethod
    def all() -> list[PieceShape]:
        """Gibt alle 21 Formen in Reihenfolge zurück."""

    @staticmethod
    def from_index(index: int) -> PieceShape | None:
        """Gibt die Form anhand ihres Index zurück, oder None."""

    def coordinates(self) -> set[Coordinate]:
        """Die normalisierten Koordinaten der Grundform."""

    def dimension(self) -> Vector:
        """Das kleinstmögliche Rechteck, das die Form umfasst."""

    def as_vectors(self) -> set[Vector]:
        """Die Form als Menge von Vektoren relativ zu (0,0)."""

    def size(self) -> int:
        """Die Anzahl der Felder, die diese Form belegt."""

    def variants(self) -> list[tuple[set[Coordinate], Rotation, bool]]:
        """
        Alle eindeutigen Varianten der Form (Rotation + Spiegelung), ohne Duplikate.

        Returns:
            Eine Liste von (Koordinatenmenge, Rotation, ist_gespiegelt)-Tupeln.
        """

    def transform(self, rotation: Rotation, should_flip: bool) -> set[Coordinate]:
        """Transformiert die Form entsprechend Rotation und Spiegelung."""

    def name(self) -> str:
        ...

class Piece:
    """Ein Spielstein mit Farbe, Position und Transformation."""

    color: Color
    kind: PieceShape
    rotation: Rotation
    is_flipped: bool
    position: Coordinate

    def __init__(
        self,
        color: Color,
        kind: PieceShape,
        rotation: Rotation,
        is_flipped: bool,
        position: Coordinate,
    ) -> None: ...
    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...
    def __hash__(self) -> int: ...

    def shape(self) -> set[Coordinate]:
        """Die normalisierte Form des Steins (gedreht/gespiegelt, nicht verschoben)."""

    def coordinates(self) -> set[Coordinate]:
        """Die tatsächlichen Koordinaten, die der Stein auf dem Feld einnimmt."""

    def transform(self, rotation: Rotation, is_flipped: bool) -> Piece:
        """Dreht/spiegelt den Stein, Position bleibt gleich."""


class Move:
    """Repräsentiert einen Zug im Spiel: entweder ein SetMove oder ein SkipMove."""

    def __eq__(self, other: object) -> bool: ...
    def __ne__(self, other: object) -> bool: ...

    @staticmethod
    def set_move(piece: Piece) -> Move:
        """Erstellt einen Zug, der den gegebenen Stein platziert."""

    @staticmethod
    def skip_move(color: Color) -> Move:
        """Erstellt einen Zug, der die aktuelle Runde für die gegebene Farbe aussetzt."""

    def get_color(self) -> Color:
        """Die Farbe, die diesen Zug getätigt hat."""

    def as_piece(self) -> Piece | None:
        """Gibt das verwendete Piece aus, wenn vorhanden"""


class GameState:
    """
    Repräsentiert den aktuellen Spielstand.

    Attribute:
        turn (int): Die Anzahl der bereits getätigten Züge.
        last_move (Move | None): Der zuletzt gespielte Zug.
        board (Board): Das aktuelle Spielfeld.
        start_piece (PieceShape): Der Spielstein, der im ersten Zug gesetzt werden muss.
        last_move_mono (dict[Color, bool]): Ob das Monomino zuletzt für jede Farbe gelegt wurde.
    """

    turn: int
    last_move: Move | None
    board: Board
    start_piece: PieceShape
    last_move_mono: dict[Color, bool]

    def __init__(
        self,
        turn: int = 0,
        last_move: Move | None = None,
        board: Board | None = None,
        start_piece: PieceShape = PieceShape.Mono,
        last_move_mono: dict[Color, bool] | None = None,
    ) -> None: ...
    def __eq__(self, other: object) -> bool: ...

    def round(self) -> int:
        """Die aktuelle Rundenzahl."""

    def undeployed_piece_shapes(self, color: Color) -> list[PieceShape]:
        """Gibt die noch nicht gesetzten Formen der gegebenen Farbe zurück."""

    def remove_undeployed_piece(self, color: Color, shape: PieceShape) -> bool:
        """Entfernt eine Form aus der Liste der noch nicht gesetzten Steine."""

    def current_color(self) -> Color:
        """Die Farbe, die aktuell am Zug ist."""

    def has_valid_colors(self) -> bool:
        """Gibt zurück, ob noch Farben im Spiel sind."""

    def is_valid_color(self, color: Color) -> bool:
        """Prüft, ob die gegebene Farbe noch im Spiel ist."""

    def remove_active_color(self) -> bool:
        """Entfernt die aktuell aktive Farbe aus dem Spiel und rückt vor."""

    def advance(self, turns: int = 1) -> bool:
        """Geht zum Zug der nächsten gültigen Farbe über."""

    def is_over(self) -> bool:
        """Gibt zurück, ob das Spiel vorbei ist."""

    def possible_moves(self) -> list[Move]:
        """
        Berechnet alle sinnvollen Züge der aktuellen Farbe.
        Enthält einen SkipMove nur, wenn kein anderer Zug möglich ist.
        """

    def get_points_for_color(self, color: Color) -> int:
        """Berechnet die Punkteanzahl für die gegebene Farbe."""

    def get_points_for_team(self, team: TeamEnum) -> int:
        """Berechnet die Punkteanzahl für das gegebene Team (Summe der Farben des Teams)."""

    def win_condition(self) -> TeamEnum | None:
        """
        Gibt das Gewinnerteam zurück, oder None bei einem Unentschieden.
        """


class GameRuleLogic:
    """Eine Sammlung an statischen Methoden, die die Spielregeln logisch umsetzen."""

    @staticmethod
    def get_points_from_undeployed(undeployed: list[PieceShape], mono_last: bool = False) -> int:
        """Berechnet den Punktestand anhand der gegebenen, nicht gelegten Formen."""

    @staticmethod
    def perform_move(game_state: GameState, move: Move) -> None:
        """
        Führt den Zug im GameState aus (mutierend).

        Raises:
            Eine der Blokus-Move-Mistake-Exceptions, wenn der Zug nicht valide ist.
        """

    @staticmethod
    def validate_move_color(game_state: GameState, move: Move) -> None:
        """
        Prüft, ob die Farbe des Zuges der aktiven Farbe entspricht.

        Raises:
            WrongColor: Wenn die Farbe nicht am Zug ist.
        """

    @staticmethod
    def validate_set_move(game_state: GameState, piece: Piece) -> None:
        """
        Prüft, ob der gegebene Stein gesetzt werden könnte.

        Raises:
            Eine der Blokus-Move-Mistake-Exceptions, wenn der Zug nicht valide ist.
        """

    @staticmethod
    def perform_set_move(game_state: GameState, piece: Piece) -> None:
        """Platziert den gegebenen Stein auf dem Spielfeld (mutierend, intern genutzt)."""

    @staticmethod
    def validate_shape(game_state: GameState, shape: PieceShape, color: Color) -> None:
        """
        Prüft, ob die Form im ersten Zug/den nachfolgenden Zügen erlaubt ist.

        Raises:
            WrongShape: Im ersten Zug, falls die falsche Form gewählt wurde.
            DuplicateShape: In folgenden Zügen, falls die Form bereits gesetzt wurde.
        """

    @staticmethod
    def is_valid_set_move(game_state: GameState, piece: Piece) -> bool:
        """Gibt zurück, ob der SetMove zulässig ist, ohne eine Exception zu werfen."""

    @staticmethod
    def validate_set_move_on_board(board: Board, piece: Piece) -> None:
        """
        Prüft, ob der Stein auf dem Board platziert werden kann (Grenzen, Überlappung, Farbregeln).

        Raises:
            OutOfBounds: Wenn der Stein nicht vollständig auf das Spielfeld passt.
            Obstructed: Wenn der Stein eine andere Farbe überlagern würde.
            TouchesSameColor: Wenn der Stein ein Feld gleicher Farbe berührt.
        """

    @staticmethod
    def validate_skip_move(game_state: GameState) -> None:
        """
        Prüft, ob die aktuelle Farbe den Zug überspringen kann.

        Raises:
            SkipFirstTurn: Wenn im ersten Zug übersprungen werden soll.
        """

    @staticmethod
    def perform_skip_move(game_state: GameState) -> None:
        """Führt einen Skip-Zug aus (validiert, mutiert aber sonst nichts)."""

    @staticmethod
    def borders_on_color(board: Board, field: Field) -> bool:
        """Prüft, ob das Feld an ein Feld gleicher Farbe angrenzt (Kante)."""

    @staticmethod
    def corners_on_color(board: Board, field: Field) -> bool:
        """Prüft, ob das Feld an die Ecke eines Feldes gleicher Farbe angrenzt."""

    @staticmethod
    def is_on_border(position: Coordinate) -> bool:
        """Prüft, ob die Position am Rand des Spielfelds liegt."""

    @staticmethod
    def is_first_move(game_state: GameState) -> bool:
        """Gibt zurück, ob sich der GameState noch in der ersten Runde befindet."""

    @staticmethod
    def get_random_start_pentomino() -> PieceShape:
        """Gibt ein zufälliges Pentomino zurück (Startstein)."""

    @staticmethod
    def remove_invalid_colors(game_state: GameState) -> None:
        """Entfernt rekursiv alle Farben, die keine Steine mehr platzieren können (mutierend)."""

    @staticmethod
    def get_all_possible_moves(game_state: GameState) -> list[Piece]:
        """Gibt eine Liste aller möglichen SetMoves zurück (inkl. möglicher Startzüge)."""

    @staticmethod
    def get_filtered_possible_moves(game_state: GameState) -> list[Piece]:
        """
        Gibt eine gefilterte Liste möglicher SetMoves zurück:
        Startzüge, dann 5 Runden nur Pentominos, danach alle.
        """

    @staticmethod
    def get_possible_start_moves(game_state: GameState, filter: bool = False) -> list[Piece]:
        """Gibt alle möglichen SetMoves für den ersten Zug zurück."""

    @staticmethod
    def get_possible_moves(game_state: GameState) -> list[Piece]:
        """Gibt alle möglichen SetMoves (ohne Startzug) zurück."""

    @staticmethod
    def get_pentomino_moves(game_state: GameState) -> list[Piece]:
        """Gibt nur die möglichen SetMoves mit Pentominos zurück."""

    @staticmethod
    def get_possible_moves_for_shape(
        game_state: GameState, shape: PieceShape, valid_fields: set[Coordinate]
    ) -> list[Piece]:
        """Gibt alle möglichen SetMoves für eine bestimmte Form zurück."""

    @staticmethod
    def get_valid_fields(board: Board, color: Color) -> set[Coordinate]:
        """Gibt alle Koordinaten zurück, auf die die gegebene Farbe einen Stein platzieren könnte."""

    @staticmethod
    def get_colored_fields(board: Board, color: Color) -> set[Coordinate]:
        """Gibt alle Koordinaten mit der gegebenen Farbe auf dem Board zurück."""


class Constants:
    """Hält globale Konstanten."""

    BOARD_LENGTH: int
    ROUND_LIMIT: int
    TOTAL_PIECE_SHAPES: int
    COLORS: int
    VALIDATE_MOVE: bool


class WrongColor(Exception):
    """Die Farbe des Zuges ist nicht an der Reihe."""

class NotOnBorder(Exception):
    """Der erste Zug muss an den Rand gesetzt werden."""

class NoSharedCorner(Exception):
    """Alle Teile müssen ein vorheriges Teil gleicher Farbe über mindestens eine Ecke berühren."""

class WrongShape(Exception):
    """Der erste Zug muss den festgelegten Spielstein setzen."""

class SkipFirstTurn(Exception):
    """Der erste Zug muss einen Stein setzen."""

class DuplicateShape(Exception):
    """Der gewählte Stein wurde bereits gesetzt."""

class OutOfBounds(Exception):
    """Der Spielstein passt nicht vollständig auf das Spielfeld."""

class Obstructed(Exception):
    """Der Spielstein würde eine andere Farbe überlagern."""

class TouchesSameColor(Exception):
    """Der Spielstein berührt ein Feld gleicher Farbe."""
