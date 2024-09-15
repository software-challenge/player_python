from enum import Enum
from typing import List, Optional

class Card(Enum):
    """
    Eine Karte, die im Spiel verwendet werden kann.

    Attributes:
        FallBack (int): Die Karte, die den Spieler zurückfallen lässt.
        HurryAhead (int): Die Karte, die den Spieler vorrücken lässt.
        EatSalad (int): Die Karte, die den Spieler Salat essen lässt.
        SwapCarrots (int): Die Karte, die die Karotten der Spieler tauscht.
    """

    FallBack: int = 0
    HurryAhead: int = 1
    EatSalad: int = 2
    SwapCarrots: int = 3

    def __init__(self) -> None: ...
    def moves(self) -> bool:
        """
        Gibt an, ob die Karte den Spieler bewegt.

        Args:
            None

        Returns:
            bool: True, wenn die Karte den Spieler bewegt, False sonst.
        """
        ...

    def perform(self, state: GameState, remaining_cards: List[Card]) -> None:
        """
        Führt die Karte aus.\n
        Diese Methode **kann** den GameState **mutieren**, unabhängig vom Erfolg des Zuges.

        Args:
            state (GameState): Der aktuelle Spielzustand.
            remaining_cards (List[Card]): Die verbleibenden Karten.

        Raises:
            HUIError: Ein Fehler, wenn die Karte nicht ausgeführt werden kann.
        """
        ...

class Advance:
    """
    Eine Klasse, die einen Vorwärtszug im Spiel darstellt.

    Attributes:
        distance (int): Die Anzahl der Felder, die der Spieler vorrücken soll.
        cards (List[Card]): Die Karten, die während des Vorgangs gespielt werden.
    """

    distance: int
    cards: List[Card]

    def __init__(self, distance: int, cards: List[Card]): ...
    def perform(self, state: GameState) -> None:
        """
        Führt den Vorwärtszug aus.\n
        Diese Methode **kann** den GameState **mutieren**, unabhängig vom Erfolg des Zuges.

        Args:
            state (GameState): Der aktuelle Zustand des Spiels.

        Raises:
            HUIError: Wenn der Vorwärtszug nicht erfolgreich ausgeführt werden kann.
        """
        ...

class EatSalad:
    def __init__(self) -> None: ...
    def perform(self, state: GameState) -> None:
        """
        Führt die Salat-Fressen Aktion aus.\n
        Diese Methode **kann** den GameState **mutieren**, unabhängig vom Erfolg des Zuges.

        Args:
            state (GameState): Der aktuelle Zustand des Spiels.

        Raises:
            HUIError: Wenn der Salat nicht erfolgreich gegessen werden kann.
        """
        ...

class ExchangeCarrots:
    amount: int

    def __init__(self, amount: int) -> None: ...
    def perform(self, state: GameState) -> None:
        """
        Führt die Karotten-Tausch-Aktion aus.\n
        Diese Methode **kann** den GameState **mutieren**, unabhängig vom Erfolg des Zuges.

        Args:
            state (GameState): Der aktuelle Zustand des Spiels.

        Raises:
            HUIError: Wenn die Karotten nicht erfolgreich getauscht werden können.
        """
        ...

class FallBack:
    def __init__(self) -> None: ...
    def perform(self, state: GameState) -> None: ...

class Field(Enum):
    Position1: int = 0
    """
    Zahlfeld
    """
    Position2: int = 1
    """
    Flaggenfeld
    """
    Hedgehog: int = 2
    """
    Igelfeld: Hierauf kann nur rückwärts gezogen werden.
    """
    Salad: int = 3
    """
    Salatfeld: Beim Betreten wird im nächsten Zug ein Salat gegessen.
    """
    Carrots: int = 4
    """
    Karottenfeld: Hier dürfen Karotten getauscht werden.
    """
    Hare: int = 5
    """
    Hasenfeld: Hier wird sofort eine Hasenkarte gespielt.
    """
    Market: int = 6
    """
    Marktfeld: Hier wird eine Hasenkarte gekauft (Variation).
    """
    Goal: int = 7
    """
    Zielfeld
    """
    Start: int = 8
    """
    Startfeld
    """

class Board:
    """
    Ein Spielbrett, das die Felder des Spiels enthält.

    Attributes:
        track (List[Field]): Die Liste der Felder, die das Spielbrett darstellen
    """

    track: list[Field]

    def __init__(self, track: list[Field]) -> None: ...
    def get_field(self, index: int) -> Optional[Field]:
        """
        Gibt das Feld am angegebenen Index zurück.

        Args:
            index (int): Der Index des Feldes, das abgerufen werden soll.

        Returns:
            Field: Das Feld am angegebenen Index, oder None, wenn außerhalb des gültigen Bereichs.
        """
        ...
    def find_field(self, field: Field, start: int, end: int) -> Optional[int]:
        """
        Findet den ersten Index des angegebenen Feldes innerhalb des angegebenen Bereichs.

        Args:
            field (Field): Das Feld, nach dem gesucht werden soll.
            start (int): Der Startindex des Bereichs, in dem gesucht werden soll.
            end (int): Der Endindex des Bereichs, in dem gesucht werden soll.

        Returns:
            int: Der Index des Feldes, wenn gefunden, oder None, wenn nicht gefunden.
        """
        ...
    def get_previous_field(self, field: Field, index: int) -> Optional[int]:
        """
        Findet die vorherige Vorkommen des angegebenen Feldes vor dem angegebenen Index.

        Args:
            field (Field): Das Feld, nach dem gesucht werden soll.
            index (int): Der Index, von dem aus gesucht werden soll.

        Returns:
            int: Der Index des vorherigen Vorkommens des Feldes, oder None, wenn nicht gefunden.
        """
        ...
    def get_next_field(self, field: Field, index: int) -> Optional[int]:
        """
        Findet das nächste Vorkommen des angegebenen Feldes nach dem angegebenen Index.

        Args:
            field (Field): Das Feld, nach dem gesucht werden soll.
            index (int): Der Index, von dem aus gesucht werden soll.

        Returns:
            int: Der Index des nächsten Vorkommens des Feldes, oder None, wenn nicht gefunden.
        """
        ...

class TeamEnum(Enum):
    One: int = 0
    """
    Team 1
    """
    Two: int = 1
    """
    Team 2
    """

    def __repr__(self) -> str: ...

class Hare:
    """
    Repräsentiert einen Hasen im Spiel.

    Attribute:
        team (TeamEnum): Das Team, dem der Hase angehört.
        position (int): Die aktuelle Position des Hasen auf dem Brett.
        salads (int): Die Anzahl der Salate, die der Hase hat.
        carrots (int): Die Anzahl der Karotten, die der Hase hat.
        last_move (Optional[Move]): Der letzte Zug, den der Hase gemacht hat.
        cards (List[Card]): Die Karten, die der Hase hat.
    """

    team: TeamEnum
    position: int
    salads: int
    carrots: int
    last_move: Optional[Move]
    cards: List[Card]

    def __init__(
        self,
        team: TeamEnum,
        cards: Optional[List[Card]] = None,
        carrots: Optional[int] = None,
        salads: Optional[int] = None,
        last_move: Optional[Move] = None,
        position: Optional[int] = None,
    ) -> None: ...
    def is_in_goal(self) -> bool:
        """
        Überprüft, ob der Hase im Ziel ist.

        Returns:
            bool: True, wenn der Hase im Ziel ist, False sonst.
        """
        ...

    def can_enter_goal(self) -> bool:
        """
        Überprüft, ob der Hase das Ziel betreten kann.

        Returns:
            bool: True, wenn der Hase das Ziel betreten kann, False sonst.
        """
        ...

    def advance_by(self, state: GameState, distance: int, cards: List[Card]) -> None:
        """
        Rückt den Hasen um eine bestimmte Entfernung vor.

        Args:
            state (GameState): Der aktuelle Spielzustand.
            distance (int): Die Entfernung, um die der Hase vorrücken soll.
            cards (List[Card]): Die Karten, die während des Vorgangs gespielt werden.

        Raises:
            HUIError: Wenn der Hase nicht vorrücken kann.
        """
        ...

    def exchange_carrots(self, state: GameState, carrots: int) -> None:
        """
        Tauscht Karotten mit dem Hasen.

        Args:
            state (GameState): Der aktuelle Spielzustand.
            carrots (int): Die Anzahl der Karotten, die getauscht werden sollen.

        Raises:
            HUIError: Wenn die Karotten nicht getauscht werden können.
        """
        ...

    def consume_carrots(self, state: GameState, carrots: int) -> None:
        """
        Verbraucht Karotten vom Hasen.

        Args:
            state (GameState): Der aktuelle Spielzustand.
            carrots (int): Die Anzahl der Karotten, die verbraucht werden sollen.

        Raises:
            HUIError: Wenn die Karotten nicht verbraucht werden können.
        """
        ...

    def eat_salad(self, state: GameState) -> None:
        """
        Lässt den Hasen einen Salat essen.

        Args:
            state (GameState): Der aktuelle Spielzustand.
        """
        ...

    def get_fall_back(self, state: GameState) -> Optional[int]:
        """
        Gibt den nächsten möglich Index zurück, an dem der Hase zurückfallen kann.

        Args:
            state (GameState): Der aktuelle Spielzustand.

        Returns:
            Optional[int]: Die Rückfallposition des Hasen, oder None, wenn nicht gefunden.
        """
        ...

    def fall_back(self, state: GameState) -> None:
        """
        Lässt den Hasen zu einer vorherigen Position zurückfallen.

        Args:
            state (GameState): Der aktuelle Spielzustand.

        Raises:
            HUIError: Wenn der Hase nicht zurückfallen kann.
        """
        ...

    def is_ahead(self, state: GameState) -> bool:
        """
        Überprüft, ob der Hase vor dem anderen Spieler ist.

        Args:
            state (GameState): Der aktuelle Spielzustand.

        Returns:
            bool: True, wenn der Hase vor dem anderen Spieler ist, False sonst.
        """
        ...

class Move:
    """
    Repräsentiert einen Zug im Spiel.

    Attribute:
        action (Advance | EatSalad | ExchangeCarrots | FallBack): Die Aktion, die der Zug ausführt.
    """

    action: Advance | EatSalad | ExchangeCarrots | FallBack

    def __init__(
        self, action: Advance | EatSalad | ExchangeCarrots | FallBack
    ) -> None: ...
    def perform(self, state: GameState) -> None:
        """
        Führt den Zug aus.

        Args:
            state (GameState): Der aktuelle Spielzustand.

        Raises:
            HUIError: Wenn der Zug nicht ausgeführt werden kann.
        """
        ...
    def __repr__(self) -> str: ...

class GameState:
    """
    Repräsentiert den aktuellen Zustand des Spiels.

    Attribute:
        board (Board): Das Spielbrett.
        turn (int): Die aktuelle Runde.
    """

    board: Board
    turn: int

    def __init__(
        self, board: Board, turn: int, player_one: Hare, player_two: Hare
    ) -> None: ...
    def perform_move(self, move: Move) -> GameState:
        """
        Führt einen Zug aus und gibt den neuen Spielzustand zurück.

        Args:
            move (Move): Der Zug, der ausgeführt werden soll.

        Returns:
            GameState: Der neue Spielzustand.

        Raises:
            HUIError: Wenn der Zug nicht ausgeführt werden kann.
        """
        ...

    def clone_current_player(self) -> Hare:
        """
        Gibt eine Kopie des aktuellen Spielers zurück.

        Returns:
            Hare: Eine Kopie des aktuellen Spielers.
        """
        ...

    def clone_other_player(self) -> Hare:
        """
        Gibt eine Kopie des anderen Spielers zurück.

        Returns:
            Hare: Eine Kopie des anderen Spielers.
        """
        ...

    def update_player(self, player: Hare) -> None:
        """
        Aktualisiert den Spieler.

        Args:
            player (Hare): Der Spieler, der aktualisiert werden soll.
        """
        ...

    def is_over(self) -> bool:
        """
        Überprüft, ob das Spiel vorbei ist.

        Returns:
            bool: True, wenn das Spiel vorbei ist, False sonst.
        """
        ...

    def possible_moves(self) -> List[Move]:
        """
        Gibt eine Liste aller möglichen Züge zurück.

        Returns:
            List[Move]: Eine Liste aller möglichen Züge.
        """
        ...

class RulesEngine:
    """
    Dient zur Überprüfung der Spielregeln.
    """

    @staticmethod
    def calculates_carrots(distance: int) -> int:
        """
        Berechnet die Anzahl der Karotten, die für einen Zug benötigt werden.

        Args:
            distance (int): Die Entfernung, die zurückgelegt werden soll.

        Returns:
            int: Die Anzahl der Karotten, die benötigt werden.
        """
        ...

    @staticmethod
    def can_exchange_carrots(board: Board, player: Hare, count: int) -> None:
        """
        Überprüft, ob ein Spieler Karotten tauschen kann.

        Args:
            board (Board): Das Spielbrett.
            player (Hare): Der Spieler, der Karotten tauschen möchte.
            count (int): Die Anzahl der Karotten, die getauscht werden sollen.

        Raises:
            HUIError: Wenn der Spieler nicht genug Karotten hat oder wenn das Feld nicht ein Karottenfeld ist.
        """
        ...

    @staticmethod
    def can_eat_salad(board: Board, player: Hare) -> None:
        """
        Überprüft, ob ein Spieler einen Salat essen kann.

        Args:
            board (Board): Das Spielbrett.
            player (Hare): Der Spieler, der einen Salat essen möchte.

        Raises:
            HUIError: Wenn der Spieler keinen Salat hat oder wenn das Feld nicht ein Salatfeld ist.
        """
        ...

    @staticmethod
    def has_to_eat_salad(board: Board, player: Hare) -> None:
        """
        Überprüft, ob ein Spieler einen Salat essen muss.

        Args:
            board (Board): Das Spielbrett.
            player (Hare): Der Spieler, der einen Salat essen muss.

        Raises:
            Exception: Wenn der Spieler nicht genug Salate hat oder wenn das Feld nicht ein Salatfeld ist.
        """
        ...

    @staticmethod
    def can_move_to(
        board: Board,
        new_position: int,
        player: Hare,
        other_player: Hare,
        cards: List[Card],
    ) -> None:
        """
        Überprüft, ob ein Spieler zu einem bestimmten Feld ziehen kann.

        Args:
            board (Board): Das Spielbrett.
            new_position (int): Die neue Position, zu der der Spieler ziehen möchte.
            player (Hare): Der Spieler, der ziehen möchte.
            other_player (Hare): Der andere Spieler.
            cards (List[Card]): Die Karten, die der Spieler spielen möchte.

        Raises:
            Exception: Wenn das Feld besetzt ist oder wenn der Spieler nicht genug Karotten hat.
        """
        ...

class PluginConstants:
    NUM_FIELDS: int
    INITIAL_SALADS: int
    INITIAL_CARROTS: int
    ROUND_LIMIT: int
