from enum import Enum
from typing import List, Optional

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
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def add_vector(self, vector: Vector) -> Coordinate:
        """
        Addiert einen Vector auf die Werte dieser Koordinate (**nicht mutierend**).

        Args:
            vector (Vector): Der Vektor.

        Returns:
            Coordinate: Ein neues Koordinatenobjekt mit den berechneten Werten.
        """
        ...

    def add_vector_mut(self, vector: Vector) -> None:
        """
        Addiert einen Vector auf die Werte dieser Koordinate (**mutierend**).

        Args:
            vector (Vector): Der Vektor.
        """
        ...

    def get_difference(self, other: Coordinate) -> Vector:
        """
        Berechnet die Differenz zwischen zwei Koordinaten Punkten als Vektor.

        Args:
            other (Coordinate): Die andere Koordinate

        Returns:
            Vector: Der Vektor zwischen den Punkten
        """
        ...

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
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def add_vector(self, other: Vector) -> Vector:
        """
        Addiert einen anderen Vector auf die Werte dieses Vektors (**nicht mutierend**).

        Args:
            other (Vector): Der andere Vektor.

        Returns:
            Vector: Ein neues Vektorobjekt mit den berechneten Werten.
        """
        ...

    def add_vector_mut(self, other: Vector) -> None:
        """
        Addiert einen anderen Vector auf die Werte dieses Vektors (**mutierend**).

        Args:
            other (Vector): Der andere Vektor.
        """
        ...

    def scale(self, scalar: int) -> Vector:
        """
        Skaliert diesen Vektor um ein gegebenes Skalar (**nicht mutierend**).

        Args:
            scalar (int): Das Skalar.

        Returns:
            Vector: Ein neues Vektorobjekt mit den berechneten Werten.
        """
        ...

    def scale_mut(self, scalar: int) -> None:
        """
        Skaliert diesen Vektor um ein gegebenes Skalar (**mutierend**).

        Args:
            scalar (int): Das Skalar.
        """
        ...

    def get_length(self) -> float:
        """
        Berechnet die Länge dieses Vektors.

        Returns:
            float: Die Länge des Vektors (in 32 bit Präzision).
        """
        ...

class Direction(Enum):
    """
    Eine Darstellung für eine normierte Richtung.<br>
    Kann in einen Vektor konvertiert werden.
    """
    
    Up: int = 0
    """
    Richtung nach oben, entspricht Vektor(0, 1).
    """
    UpRight: int = 1
    """
    Richtung nach oben-rechts, entspricht Vektor(1, 1).
    """
    Right: int = 2
    """
    Richtung nach rechts, entspricht Vektor(1, 0).
    """
    DownRight: int = 3
    """
    Richtung nach unten-rechts, entspricht Vektor(1, -1).
    """
    Down: int = 4
    """
    Richtung nach unten, entspricht Vektor(0, -1).
    """
    DownLeft: int = 5
    """
    Richtung nach unten-links, entspricht Vektor(-1, -1).
    """
    Left: int = 6
    """
    Richtung nach links, entspricht Vektor(-1, 0).
    """
    UpLeft: int = 7
    """
    Richtung nach oben-links, entspricht Vektor(-1, 1).
    """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    @staticmethod
    def from_vector(vector: Vector) -> Optional[Direction]:
        """
        Wandelt einen Vektor in eine der 8 Richtungen um, insofern der Vektor exakt der Richtung entspricht.

        Args:
            vector (Vector): Der Vektor, der konvertiert werden soll.

        Returns:
            Optional[Direction]: Die Richtung oder None, wenn der Vektor nicht direkt übersetzt werden kann.
        """
        ...
    
    @staticmethod
    def all_directions() -> List[Direction]:
        """
        Gibt eine Liste aller 8 Richtungen aus.<br>
        Der erste Wert ist oben und alle weiteren folgen im Uhrzeigersinn.
        
        Returns:
            List[Direction]: Die Liste der Richtungen.
        """
        ...

    def to_vector(self) -> Vector:
        """
        Wandelt die Richtung in den entsprechenden Vektor um.

        Returns:
            Vector: Der Richtungsvektor.
        """
        ...

    def to_mirrored(self) -> Direction:
        """
        Spiegelt die gegebene Richtung.<br>
        Beispiel: Up -> Down.

        Returns:
            Direction: Die neue Richtung.
        """
        ...

class FieldType(Enum):
    """
    Stellt alle verfügbaren Feldtypen dar.
    """

    OneS: int = 0
    """
    Der kleine Fisch von Spieler 1.
    """
    OneM: int = 1
    """
    Der mittlere Fisch von Spieler 1.
    """
    OneL: int = 2
    """
    Der große Fisch von Spieler 1.
    """
    TwoS: int = 3
    """
    Der kleine Fisch von Spieler 2.
    """
    TwoM: int = 4
    """
    Der mittlere Fisch von Spieler 2.
    """
    TwoL: int = 5
    """
    Der große Fisch von Spieler 2.
    """
    Squid: int = 6
    """
    Der Kraken.
    """
    Empty: int = 7
    """
    Alle anderen unbesetzten Felder.
    """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def get_value(self) -> int:
        """
        Gibt die entsprechende Wertigkeit eines Feldes zurück.<br>
        Für Fische je nachdem 1-3, für die anderen Felder 0.
        
        Returns:
            int: Der Wert des Feldes.
        """
        ...

    def get_team(self) -> Optional[TeamEnum]:
        """
        Gibt für ein Fischfeld aus, zu welchem Team dieser Fisch gehört.

        Returns:
            Optional[TeamEnum]: Das Team, zudem der Fisch gehört, oder None, wenn das Feld kein Fisch ist.
        """
        ...

    @staticmethod
    def all_field_types() -> List[FieldType]:
        """
        Gibt eine Liste aller Feldtypen aus.<br>
        
        Returns:
            List[Direction]: Die Liste der Richtungen.
        """
        ...

class TeamEnum(Enum):
    """
    Eine Darstellung für die beiden Teams
    """

    One = 0
    """
    Team 1
    """
    Two = 1
    """
    Team 2
    """

    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def get_fish_types(self) -> List[FieldType]:
        """
        Gibt eine Liste aller Fischtypen des Teams aus.

        Returns:
            List[FieldType]: Die Liste der Feldtypen.
        """
        ...

    def opponent(self) -> TeamEnum:
        """
        Gibt den Gegner dieses Teams an.

        Return:
            TeamEnum: Das Gegnerteam.
        """
        ...

class Board:
    """
    Ein Spielbrett, das die Felder des Spiels enthält.

    Das Feld unten-links hat Koordinate (0, 0) und das Feld oben-rechts ist an Position (9, 9).<br>
    Gleichzeitig bedeutet das, dass map[0] auch die unterste Zeile des Spielfeldes ist.

    Attributes:
        map (List[List[Field]]): Die 2 dimensionale Liste der Felder, die das Spielbrett darstellen.<br>
    """

    map: List[List[FieldType]]

    def __init__(self, map: List[List[FieldType]]) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def get_field(self, position: Coordinate) -> Optional[FieldType]:
        """
        Gibt das Feld an der gegebenen Koordinate zurück.

        Args:
            position (Coordinate): Die Position des Feldes, das abgerufen werden soll.

        Returns:
            Field: Das Feld an der gegebenen Koordinate, oder None, wenn außerhalb des gültigen Bereichs.
        """
        ...

    def get_fields_by_type(self, field: FieldType) -> List[Coordinate]:
        """
        Gibt eine Liste aller Koordinaten zurück, auf dem der angegebene Feld-Typ zu finden ist.

        Args:
            field (FieldType): Der Feld-Typ, nachdem gesucht werden soll.

        Returns:
            List[Coordinate]: Die Liste der Koordinaten.
        """
        ...

    def get_fields_in_direction(self, position: Coordinate, direction: Direction) -> List[FieldType]:
        """
        Gibt eine Liste aller Feld-Typen zurück, die in einer Richtung liegen.<br>
        Dabei wird als Ausgangspunkt eine Koordinate genommen und dazu die Richtung, in die aufgelistet werden soll.

        **Achtung**: Das Feld der Ausgangskoordinate wird *nicht* beachtet und ausgegeben.<br>
        Wenn die Startkoordinate nicht im Spielfeld liegt, wird eine leere Liste zurückgegeben.

        Args:
            position (Coordinate): Die Ausgangskoordinate.
            direction (Direction): Die Richtung.

        Returns:
            List[FieldType]: Die Liste der Felder.
        """

    def get_fields_on_line(self, position: Coordinate, direction: Direction) -> List[FieldType]:
        """
        Gibt eine Liste aller Feld-Typen zurück, die auf einer Gerade liegen.<br>
        Die Gerade wird aufgespannt durch eine Koordinate und einen Richtungsvektor.

        Das Ergebnis wird in Richtung des "Vektorpfeils" abgelesen.<br>
        Wenn die Startkoordinate nicht im Spielfeld liegt, wird eine leere Liste zurückgegeben.

        Args:
            position (Coordinate): Die Startkoordinate für die Gerade.
            direction (Direction): Der aufspannende Richtungsvektor.

        Returns:
            List[FieldType]: Die Liste der Felder.
        """
        ...

    def get_fish_on_line(self, position: Coordinate, direction: Direction) -> List[FieldType]:
        """
        Funktioniert ähnlich wie *Board.get_fields_on_line()*,
        gibt aber nur die Feldtypen, die Fische sind, auf einer Geraden als Liste aus.

        Args:
            position (Coordinate): Die Startkoordinate für die Gerade.
            direction (Direction): Der aufspannende Richtungsvektor.

        Returns:
            List[FieldType]: Die Liste der Fisch-Felder.
        """
        ...

class Move:
    """
    Repräsentiert einen Zug im Spiel.

    Attribute:
        start (Coordinate): Die Koordinate, von wo aus ein Fisch bewegt werden soll.
        direction (Direction): Die Richtung, in die der Fisch schwimmt.
    """

    start: Coordinate
    direction: Direction

    def __init__(self, start: Coordinate, direction: Direction) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

class GameState:
    """
    Repräsentiert einen Spielstand.

    Attribute:
        board (Board): Das Spielbrett.
        turn (int): Die aktuelle Runde.
        last_move (Optional[Move]): Der zuletzt ausgeführte Zug.
    """

    board: Board
    turn: int
    last_move: Optional[Move]

    def __init__(self, board: Board, turn: int, last_move: Optional[Move]) -> None: ...
    def __str__(self) -> str: ...
    def __repr__(self) -> str: ...

    def possible_moves_for(self, start: Coordinate) -> List[Move]:
        """
        Berechnet alle Züge, die aus der aktuellen Spielposition für den Fisch an der Koordinate möglich sind.
        
        Args:
            start (Coordinate): Die Position des gewählten Fisch.

        Returns:
            List[Move]: Die Liste der Züge.
        """
        ...

    def possible_moves(self) -> List[Move]:
        """
        Berechnet alle Züge, die aus der aktuellen Spielposition für den aktuellen Spieler möglich sind.

        Returns:
            List[Move]: Die Liste der Züge.
        """
        ...

    def perform_move(self, move: Move) -> GameState:
        """
        Führt den gegebenen Zug auf dem Spielstand aus, insofern dieser ausführbar ist (**nicht mutierend**).
        Dabei wird *kein* Zug an den Spielserver übermittelt.

        Args:
            move_ (Move): Der zuverwendene Zug.

        Returns:
            Gamestate: Der neue Spielstand.

        Raises:
            PiranhasError: Wenn der Zug nicht valide ist.
        """
        ...

    def perform_move_mut(self, move: Move) -> None:
        """
        Führt den gegebenen Zug auf dem Spielstand aus, insofern dieser ausführbar ist (**mutierend**).
        Dabei wird *kein* Zug an den Spielserver übermittelt.

        Args:
            move_ (Move): Der zuverwendene Zug.

        Raises:
            PiranhasError: Wenn der Zug nicht valide ist.
        """
        ...

class RulesEngine:
    """
    Stellt Methoden, die zur Überprüfung der Spielregeln dienen.
    """

    @staticmethod
    def move_distance(board: Board, move_: Move) -> int:
        """
        Gibt die Länge / Anzahl der Felder von einem Zug auf dem Spielfeld zurück.

        Args:
            board (Board): Das Spielfeld, auf dem die Länge berechnet werden soll.
            move_ (Move): Der zuverwendene Zug.

        Returns:
            int: Die Länge.
        """
        ...


    @staticmethod
    def target_position(board: Board, move_: Move) -> Coordinate:
        """
        Gibt die Koordinate zurück, auf der ein Fisch landen würde, wenn man den Zug ausführt.

        Es wird nicht berücksichtigt, ob diese Koordinate im Spielfeld ist.

        Args:
            board (Board): Das Spielfeld, auf dem der Zug berechnet werden soll.
            move_ (Move): Der zuverwendene Zug.

        Returns:
            Coordinate: Die Koordinate.
        """
        ...

    @staticmethod
    def is_in_bounds(coordinate: Coordinate) -> bool:
        """
        Gibt einen Wahrheitswert zurück, ob eine Position in dem (Standard-) Spielfeld (10x10) liegt.

        Args:
            coordinate (Coordinate): Die Position

        Returns:
            bool: Ob die Koordinate im Feld ist.
        """
        ...

    @staticmethod
    def can_execute_move(board: Board, move_: Move) -> None:
        """
        Prüft, ob ein Zug auf dem Board nach den Regeln durchgeführt werden könnte.<br>
        Dabei ist nicht relevant, welcher Spieler gerade tatsächlich dran wäre.

        Gibt keinen Wert zurück, sondern wirft eine Fehlermeldung, falls der Zug nicht valide ist.

        Args:
            board (Board): Das Spielfeld.
            move_ (Move): Der Zug, der geprüft werden soll.

        Raises:
            PiranhasError: Wenn der Zug nicht valide ist.
        """
        ...

    @staticmethod 
    def get_team_on_turn(turn: int) -> TeamEnum:
        """
        Berechnet anhand der Zugzahl, welcher Spieler dran sein müsste.<br>
        Es wird nicht beachtet, ob die Zahl kleiner 0 oder größer 59 ist.

        Args:
            turn (int): Die Zugzahl

        Returns:
            TeamEnum: Das Team, was dran ist.
        """
        ...

class PluginConstants:
    """
    Hält globale Konstanten.
    """

    BOARD_WIDTH: int
    BOARD_HEIGHT: int

    ROUND_LIMIT: int
