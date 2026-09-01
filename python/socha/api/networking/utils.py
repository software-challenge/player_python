from typing import Optional

from socha import _socha
from socha.api.protocol.protocol import (
    Board,
    Data,
    LastMove,
    LastMoveMono,
    Piece,
    Position,
    Room,
    State,
)


# SCREAMING_SNAKE_CASE (Server) <-> CamelCase-Variantenname (Rust/_socha)
_SHAPE_NAME_MAP = {
    "MONO": "Mono",
    "DOMINO": "Domino",
    "TRIO_L": "TrioL",
    "TRIO_I": "TrioI",
    "TETRO_O": "TetroO",
    "TETRO_T": "TetroT",
    "TETRO_I": "TetroI",
    "TETRO_L": "TetroL",
    "TETRO_Z": "TetroZ",
    "PENTO_L": "PentoL",
    "PENTO_T": "PentoT",
    "PENTO_V": "PentoV",
    "PENTO_S": "PentoS",
    "PENTO_Z": "PentoZ",
    "PENTO_I": "PentoI",
    "PENTO_P": "PentoP",
    "PENTO_W": "PentoW",
    "PENTO_U": "PentoU",
    "PENTO_R": "PentoR",
    "PENTO_X": "PentoX",
    "PENTO_Y": "PentoY",
}
_SHAPE_NAME_MAP_REVERSE = {v: k for k, v in _SHAPE_NAME_MAP.items()}


def map_shape(name: str) -> _socha.PieceShape:
    """
    Wandelt einen Server-Formnamen (z.B. 'PENTO_Y') in die entsprechende
    _socha.PieceShape-Variante (z.B. PieceShape.PentoY) um.
    """
    return getattr(_socha.PieceShape, _SHAPE_NAME_MAP[name])


def map_shape_to_string(shape: _socha.PieceShape) -> str:
    """
    Wandelt eine _socha.PieceShape-Variante zurück in den Server-Formnamen.
    """
    return _SHAPE_NAME_MAP_REVERSE[shape.name()]


def map_color(name: str) -> _socha.Color:
    """
    Color-Varianten matchen 1:1 zwischen Server und _socha (beide UPPERCASE).
    """
    return getattr(_socha.Color, name)


def map_rotation(name: str) -> _socha.Rotation:
    """
    Rotation-Varianten matchen 1:1 zwischen Server und _socha (beide UPPERCASE).
    """
    return getattr(_socha.Rotation, name)


def map_piece(piece: Piece) -> _socha.Piece:
    """
    Konvertiert ein protokoll Piece-Objekt in ein _socha.Piece-Objekt.
    """
    return _socha.Piece(
        color=map_color(piece.color),
        kind=map_shape(piece.kind),
        rotation=map_rotation(piece.rotation),
        is_flipped=piece.is_flipped,
        position=_socha.Coordinate(piece.position.x, piece.position.y),
    )


def map_piece_to_protocol(piece: _socha.Piece) -> Piece:
    """
    Konvertiert ein _socha.Piece-Objekt zurück in ein protokoll Piece-Objekt,
    zum Versenden an den Server.
    """
    return Piece(
        color=piece.color.name(),
        kind=map_shape_to_string(piece.kind),
        rotation=piece.rotation.name(),
        is_flipped=piece.is_flipped,
        position=Position(x=piece.position.x, y=piece.position.y),
    )


def map_last_move(protocol_last_move: Optional[LastMove]) -> Optional[_socha.Move]:
    """
    Konvertiert das lastMove-Element eines State in ein _socha.Move-Objekt.
    Kann entweder ein SetMove (piece gesetzt) oder ein SkipMove (color gesetzt) sein.
    """
    if protocol_last_move is None:
        return None
    if protocol_last_move.piece is not None:
        return _socha.Move.set_move(map_piece(protocol_last_move.piece))
    if protocol_last_move.color is not None:
        return _socha.Move.skip_move(map_color(protocol_last_move.color))
    return None


def map_last_move_mono(last_move_mono: Optional[LastMoveMono]) -> dict:
    """
    Konvertiert das lastMoveMono-Element (XStream-Standard-Map-Serialisierung)
    in ein Python-Dict[Color, bool].

    UNBESTÄTIGT: In allen bisherigen Aufzeichnungen war dieses Element leer,
    daher basiert die Struktur auf einer fundierten Annahme über XStreams
    Standardverhalten für unkonvertierte HashMaps.
    """
    if last_move_mono is None:
        return {}
    result = {}
    for entry in last_move_mono.entry:
        if entry.color is not None:
            result[map_color(entry.color)] = bool(entry.value)
    return result


def map_board(protocol_board: Board) -> _socha.Board:
    """
    Baut ein volles Board auf. Der Server sendet nur belegte Felder,
    daher wird zunächst ein leeres Board erzeugt und dann überschrieben.
    """
    board_map = _socha.Board.random_fields()  # bereits vollständig EMPTY

    for f in protocol_board.field_value:
        color = map_color(f.content)
        coord = _socha.Coordinate(f.x, f.y)
        board_map[f.y][f.x] = _socha.Field(coord, color)

    return _socha.Board(map=board_map)


def handle_move(move: _socha.Move) -> Data:
    """
    Konvertiert einen _socha.Move (SetMove oder SkipMove) in das ausgehende
    Data-Paket, das an den Server gesendet wird.

    UNBESTÄTIGT für SkipMove: Es wurde in keiner Aufzeichnung tatsächlich
    ein SkipMove beobachtet. Die Struktur (color als Kindelement) basiert
    auf der Kotlin-Deklaration von SkipMove.color (kein @XStreamAsAttribute).
    """
    piece = move.as_piece()
    if piece is not None:
        return Data(
            class_value="sc.plugin2027.SetMove",
            piece=map_piece_to_protocol(piece),
        )
    else:
        return Data(
            class_value="sc.plugin2027.SkipMove",
            skip_color=move.get_color().name(),
        )


def message_to_state(message: Room) -> _socha.GameState:
    """
    Konstruiert einen vollständigen GameState aus der vom Server empfangenen
    Nachricht, inklusive aller noch nicht gesetzten Formen pro Farbe und
    der aktuell gültigen Farben.
    """
    state: State = message.data.class_binding

    return _socha.GameState(
        turn=state.turn,
        last_move=map_last_move(state.last_move),
        board=map_board(state.board),
        start_piece=map_shape(state.start_piece),
        last_move_mono=map_last_move_mono(state.last_move_mono),
        blue_shapes=[map_shape(s) for s in state.blue_shapes.shape]
        if state.blue_shapes
        else [],
        yellow_shapes=[map_shape(s) for s in state.yellow_shapes.shape]
        if state.yellow_shapes
        else [],
        red_shapes=[map_shape(s) for s in state.red_shapes.shape]
        if state.red_shapes
        else [],
        green_shapes=[map_shape(s) for s in state.green_shapes.shape]
        if state.green_shapes
        else [],
        valid_colors=[map_color(c) for c in state.valid_colors.color]
        if state.valid_colors
        else [],
    )