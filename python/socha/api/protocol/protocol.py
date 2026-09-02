# ruff: noqa: UP045

from dataclasses import dataclass, field
from typing import Optional

from socha._socha import TeamEnum
from socha.api.protocol.protocol_packet import (
    AdminLobbyRequest,
    LobbyRequest,
    ProtocolPacket,
    ResponsePacket,
)
from socha.api.protocol.room_message import (
    ObservableRoomMessage,
    RoomMessage,
    RoomOrchestrationMessage,
)


@dataclass
class Position:
    """
    Eine Position auf dem Spielbrett, verschachtelt innerhalb eines Piece-Elements.
    """

    class Meta:
        name = 'position'

    x: Optional[int] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )
    y: Optional[int] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )


@dataclass
class Piece:
    """
    Ein Spielstein mit Farbe, Form, Rotation, Spiegelung und Position.
    """

    class Meta:
        name = 'piece'

    color: Optional[str] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )
    kind: Optional[str] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )
    rotation: Optional[str] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )
    is_flipped: Optional[bool] = field(
        default=None,
        metadata={
            'name': 'isFlipped',
            'type': 'Attribute',
        },
    )
    position: Optional[Position] = field(
        default=None,
        metadata={'type': 'Element'},
    )


@dataclass
class LastMove:
    class Meta:
        name = 'lastMove'

    class_binding: Optional[object] = field(default=None)
    class_value: Optional[str] = field(
        default=None,
        metadata={
            'name': 'class',
            'type': 'Attribute',
            'required': True,
        },
    )
    piece: Optional[Piece] = field(
        default=None,
        metadata={'type': 'Element'},
    )
    # Für SkipMove: SkipMove.color hat kein @XStreamAsAttribute in Kotlin,
    # wird daher als Kindelement serialisiert (nicht als Attribut).
    color: Optional[str] = field(
        default=None,
        metadata={'type': 'Element'},
    )


@dataclass
class Field:
    class Meta:
        name = 'field'

    x: Optional[int] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )
    y: Optional[int] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )
    content: Optional[str] = field(
        default=None,
        metadata={'type': 'Attribute'},
    )


@dataclass
class Board:
    """
    Das Spielbrett. Enthält nur belegte Felder;
    leere Felder werden vom Server nicht mitgeschickt.
    """

    class Meta:
        name = 'board'

    field_value: list[Field] = field(
        default_factory=list,
        metadata={
            'name': 'field',
            'type': 'Element',
        },
    )


@dataclass
class ShapeList:
    """
    Wiederverwendet für blueShapes / yellowShapes / redShapes / greenShapes.
    Enthält die noch nicht gesetzten Formen einer Farbe.
    """

    shape: list[str] = field(
        default_factory=list,
        metadata={'type': 'Element'},
    )


@dataclass
class ColorList:
    class Meta:
        name = 'validColors'

    color: list[str] = field(
        default_factory=list,
        metadata={'type': 'Element'},
    )


@dataclass
class LastMoveMonoEntry:
    """
    Ein Eintrag der lastMoveMono-HashMap, wie von XStreams
    Standard-Map-Serialisierung erzeugt (kein eigener Converter registriert).

    UNBESTÄTIGT: Struktur beruht auf XStream-Standardverhalten,
    da lastMoveMono in allen Aufzeichnungen bisher leer war.
    """

    class Meta:
        name = 'entry'

    color: Optional[str] = field(
        default=None,
        metadata={
            'name': 'sc.plugin2027.Color',
            'type': 'Element',
        },
    )
    value: Optional[bool] = field(
        default=None,
        metadata={
            'name': 'boolean',
            'type': 'Element',
        },
    )


@dataclass
class LastMoveMono:
    class Meta:
        name = 'lastMoveMono'

    entry: list[LastMoveMonoEntry] = field(
        default_factory=list,
        metadata={'type': 'Element'},
    )


@dataclass
class State(ObservableRoomMessage):
    class Meta:
        name = 'state'

    class_value: Optional[str] = field(
        default=None,
        metadata={
            'name': 'class',
            'type': 'Attribute',
            'required': True,
        },
    )
    start_team: Optional[str] = field(
        default=None,
        metadata={
            'name': 'startTeam',
            'type': 'Attribute',
            'required': True,
        },
    )
    turn: Optional[int] = field(
        default=None,
        metadata={
            'type': 'Attribute',
            'required': True,
        },
    )
    start_piece: Optional[str] = field(
        default=None,
        metadata={
            'name': 'startPiece',
            'type': 'Attribute',
            'required': True,
        },
    )
    round: Optional[int] = field(
        default=None,
        metadata={
            'type': 'Attribute',
            'required': True,
        },
    )
    last_move: Optional[LastMove] = field(
        default=None,
        metadata={
            'name': 'lastMove',
            'type': 'Element',
        },
    )
    board: Optional[Board] = field(
        default=None,
        metadata={
            'type': 'Element',
            'required': True,
        },
    )
    last_move_mono: Optional[LastMoveMono] = field(
        default=None,
        metadata={
            'name': 'lastMoveMono',
            'type': 'Element',
        },
    )
    blue_shapes: Optional[ShapeList] = field(
        default=None,
        metadata={
            'name': 'blueShapes',
            'type': 'Element',
        },
    )
    yellow_shapes: Optional[ShapeList] = field(
        default=None,
        metadata={
            'name': 'yellowShapes',
            'type': 'Element',
        },
    )
    red_shapes: Optional[ShapeList] = field(
        default=None,
        metadata={
            'name': 'redShapes',
            'type': 'Element',
        },
    )
    green_shapes: Optional[ShapeList] = field(
        default=None,
        metadata={
            'name': 'greenShapes',
            'type': 'Element',
        },
    )
    valid_colors: Optional[ColorList] = field(
        default=None,
        metadata={
            'name': 'validColors',
            'type': 'Element',
        },
    )


@dataclass
class Player:
    class Meta:
        name = 'player'

    name: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
            'required': True,
        },
    )
    team: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
            'required': True,
        },
    )


@dataclass
class OriginalRequest(ProtocolPacket):
    class Meta:
        name = 'originalRequest'

    class_value: Optional[str] = field(
        default=None,
        metadata={
            'name': 'class',
            'type': 'Attribute',
        },
    )
    reservation_code: Optional[str] = field(
        default=None,
        metadata={
            'name': 'reservationCode',
            'type': 'Attribute',
        },
    )


@dataclass
class Errorpacket(ProtocolPacket):
    class Meta:
        name = 'errorpacket'

    message: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
        },
    )
    original_request: Optional[OriginalRequest] = field(
        default=None,
        metadata={
            'name': 'originalRequest',
            'type': 'Element',
        },
    )


@dataclass
class Left(ProtocolPacket):
    """
    If the game is over the server will _send this message to the clients and closes the connection afterward.
    """

    class Meta:
        name = 'left'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )


@dataclass
class MoveRequest(RoomMessage):
    """
    Request a client to _send a Move.
    """


@dataclass
class Close(ProtocolPacket):
    """
    Is sent by one party immediately before this party closes the communication connection and should make the
    receiving party also close the connection.

    This should not be sent manually, the XFluxClient will automatically _send it when stopped.
    """

    class Meta:
        name = 'close'


@dataclass
class Authenticate(AdminLobbyRequest):
    """
    Authenticates a client as administrator to _send AdminLobbyRequest`s.
    *Is not answered if successful.*
    """

    class Meta:
        name = 'authenticate'

    password: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
        },
    )


@dataclass
class Cancel(AdminLobbyRequest):
    """
    Deletes the GameRoom and cancels the Game within.
    """

    class Meta:
        name = 'cancel'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )


@dataclass
class JoinedGameRoom(ObservableRoomMessage):
    """
    Sent to all administrative clients after a player joined a GameRoom via a JoinRoomRequest.
    """

    class Meta:
        name = 'joinedGameRoom'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )
    player_count: Optional[int] = field(
        default=None,
        metadata={
            'name': 'playerCount',
            'type': 'Attribute',
        },
    )


@dataclass
class Observe(AdminLobbyRequest):
    """
    Sent to client as response to successfully joining a GameRoom as Observer.
    """

    class Meta:
        name = 'observe'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )


@dataclass
class Pause(AdminLobbyRequest):
    """
    Indicates to observers that the game has been (un)paused.
    """

    class Meta:
        name = 'pause'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )
    pause: Optional[bool] = field(
        default=None,
        metadata={
            'type': 'Attribute',
        },
    )


@dataclass
class Slot(RoomOrchestrationMessage):
    """
    Slots for a game which contains the player's name and its attributes.
    """

    class Meta:
        name = 'slot'

    display_name: Optional[str] = field(
        default=None,
        metadata={
            'name': 'displayName',
            'type': 'Attribute',
        },
    )
    can_timeout: Optional[bool] = field(
        default=None,
        metadata={
            'name': 'canTimeout',
            'type': 'Attribute',
        },
    )
    reserved: Optional[bool] = field(
        default=None,
        metadata={
            'type': 'Attribute',
        },
    )


@dataclass
class Step(RoomOrchestrationMessage):
    """
    When the client is authenticated as administrator,
    it can _send this step request to the server to advance the game for one move.
    This is not possible if the game is not paused.
    """

    class Meta:
        name = 'step'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )


@dataclass
class Prepare(RoomOrchestrationMessage):
    """
    When the client is authenticated as administrator,
    it can _send this request to prepare the room for the game.
    """

    class Meta:
        name = 'prepare'

    game_type: Optional[str] = field(
        default=None,
        metadata={
            'name': 'gameType',
            'type': 'Attribute',
        },
    )
    pause: Optional[bool] = field(
        default=None,
        metadata={
            'type': 'Attribute',
        },
    )
    slot: list[Slot] = field(
        default_factory=list,
        metadata={
            'type': 'Element',
        },
    )


@dataclass
class Join(LobbyRequest):
    """
    Joins any room that is open.
    If no room is open,
    a new room is created by the server.
    """

    class Meta:
        name = 'join'


@dataclass
class JoinPrepared(LobbyRequest):
    """
    Join a prepared room with a reservation code.
    """

    class Meta:
        name = 'joinPrepared'

    reservation_code: Optional[str] = field(
        default=None,
        metadata={
            'name': 'reservationCode',
            'type': 'Attribute',
        },
    )


@dataclass
class JoinRoom(LobbyRequest):
    """
    To join a room with a `room_id`.
    """

    class Meta:
        name = 'joinRoom'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )


@dataclass
class Fragment:
    """
    This holds the fragments of a winning definition.
    """

    class Meta:
        name = 'fragment'

    name: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
        },
    )
    aggregation: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    relevant_for_ranking: Optional[bool] = field(
        default=None,
        metadata={
            'name': 'relevantForRanking',
            'type': 'Element',
        },
    )


@dataclass
class Joined(ResponsePacket):
    """
    Sent to all clients after a player joined a GameRoom via a Join Request.
    """

    class Meta:
        name = 'joined'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )


@dataclass
class Score:
    """
    Score of the players when the game has ended.
    """

    class Meta:
        name = 'score'

    part: list[int] = field(
        default_factory=list,
        metadata={
            'type': 'Element',
            'min_occurs': 1,
        },
    )


@dataclass
class Winner:
    class Meta:
        name = 'winner'

    team: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
            'required': True,
        },
    )
    regular: Optional[bool] = field(
        default=None,
        metadata={
            'type': 'Attribute',
            'required': True,
        },
    )
    reason: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
            'required': True,
        },
    )


@dataclass
class Definition:
    """
    The definition of a result of a game.
    If for instance one player made an error move, the game is over and the other player wins,
    the definition will tell that the other player wins, because of the error.
    """

    class Meta:
        name = 'definition'

    fragment: list[Fragment] = field(
        default_factory=list,
        metadata={
            'type': 'Element',
        },
    )


@dataclass
class Entry:
    """
    Is _send when a game is won by one of the players.
    This element contains the winning player and the score of the player.
    """

    class Meta:
        name = 'entry'

    player: Optional[Player] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    score: Optional[Score] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )


@dataclass
class Scores:
    """
    Then result of a game when its over.
    """

    class Meta:
        name = 'scores'

    entry: list[Entry] = field(
        default_factory=list,
        metadata={
            'type': 'Element',
        },
    )


@dataclass
class WelcomeMessage(RoomOrchestrationMessage):
    """
    Welcome message is sent to the client when the client joins the room.
    In this message the server tells the client which team it is.
    """

    team: TeamEnum


@dataclass
class Result(ObservableRoomMessage):
    """
    Result of a game.
    This will the server _send after a game is finished.
    """

    definition: Definition
    scores: Scores
    winner: Winner


@dataclass
class OriginalMessage:
    """
    The original message that was sent by the client.
    Is sent by the server if an error occurs.

    UNBESTÄTIGT für Blokus: Struktur übernommen aus dem alten Piranhas-Format
    mit from_/direction. Da wir keine echte error-Nachricht aufgezeichnet haben,
    ist unklar, ob der Server hier stattdessen ein <piece>- oder <color>-Element
    verwendet, analog zu Data. Passe ggf. an, sobald eine echte error-Nachricht
    für einen Blokus-Move vorliegt.
    """

    class Meta:
        name = 'originalMessage'

    class_value: Optional[str] = field(
        default=None,
        metadata={
            'name': 'class',
            'type': 'Attribute',
            'required': True,
        },
    )
    piece: Optional[Piece] = field(
        default=None,
        metadata={'type': 'Element'},
    )
    color: Optional[str] = field(
        default=None,
        metadata={'type': 'Element'},
    )


@dataclass
class Error:
    """
    This sends the server when the client sent a erroneous message.
    """

    message: str
    originalMessage: OriginalMessage


@dataclass
class Data:
    class Meta:
        name = 'data'

    class_value: Optional[str] = field(
        default=None,
        metadata={
            'name': 'class',
            'type': 'Attribute',
            'required': True,
        },
    )
    class_binding: Optional[object] = field(default=None)
    definition: Optional[Definition] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    original_message: Optional[OriginalMessage] = field(
        default=None,
        metadata={
            'name': 'originalMessage',
            'type': 'Element',
        },
    )
    scores: Optional[Scores] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    winner: Optional[Winner] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    state: Optional[State] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    # Nur für welcomeMessage: color="ONE"/"TWO" (TeamEnum), als Attribut.
    color: Optional[str] = field(
        default=None,
        metadata={
            'type': 'Attribute',
        },
    )
    # Für ausgehenden SetMove.
    piece: Optional[Piece] = field(
        default=None,
        metadata={'type': 'Element'},
    )
    # Für ausgehenden SkipMove: eigenes Feld, da 'color' oben schon als
    # Attribut für welcomeMessage belegt ist. SkipMove.color ist ein
    # Kindelement (kein @XStreamAsAttribute in Kotlin).
    skip_color: Optional[str] = field(
        default=None,
        metadata={
            'name': 'color',
            'type': 'Element',
        },
    )


@dataclass
class Room(ProtocolPacket):
    class Meta:
        name = 'room'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
            'required': True,
        },
    )
    data: Optional[Data] = field(
        default=None,
        metadata={
            'type': 'Element',
            'required': True,
        },
    )


@dataclass
class Observed(RoomOrchestrationMessage):
    class Meta:
        name = 'observed'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )


@dataclass
class Prepared(RoomOrchestrationMessage):
    class Meta:
        name = 'prepared'

    room_id: Optional[str] = field(
        default=None,
        metadata={
            'name': 'roomId',
            'type': 'Attribute',
        },
    )
    reservation: list[str] = field(
        default_factory=list,
        metadata={
            'type': 'Element',
        },
    )


@dataclass
class Protocol:
    """
    This is the root element of the protocol.
    Even it's in here it will never be called,
    because the children of this root element have to be handled separately.
    """

    class Meta:
        name = 'protocol'

    authenticate: Optional[Authenticate] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    joined_game_room: Optional[JoinedGameRoom] = field(
        default=None,
        metadata={
            'name': 'joinedGameRoom',
            'type': 'Element',
        },
    )
    prepare: Optional[Prepare] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    observe: Optional[Observe] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    pause: Optional[Pause] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    step: Optional[Step] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    cancel: Optional[Cancel] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    join: Optional[Join] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    joined: Optional[Joined] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    room: list[Room] = field(
        default_factory=list,
        metadata={
            'type': 'Element',
        },
    )
    prepared: Optional[Prepared] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )
    observed: Optional[Observed] = field(
        default=None,
        metadata={
            'type': 'Element',
        },
    )