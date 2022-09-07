from dataclasses import dataclass, field
from typing import List, Optional, Union

from socha.api.plugin.penguins import Team
from socha.api.protocol.protocol_packet import AdminLobbyRequest, ResponsePacket, ProtocolPacket, LobbyRequest
from socha.api.protocol.room_message import RoomOrchestrationMessage, RoomMessage, \
    ObservableRoomMessage


@dataclass
class Left(ProtocolPacket):
    """
    If the game is over the server will send this message to the clients and closes the connection afterwards.
    """

    class Meta:
        name = "left"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )


@dataclass
class MoveRequest(RoomMessage):
    """
    Request a client to send a Move.
    """


@dataclass
class Close(ProtocolPacket):
    """
    Is sent by one party immediately before this party closes the communication connection and should make the
    receiving party also close the connection.

    This should not be sent manually, the XFluxClient will automatically send it when stopped.
    """

    class Meta:
        name = "close"


@dataclass
class Authenticate(AdminLobbyRequest):
    """
    Authenticates a client as administrator to send AdminLobbyRequest`s. \n
    *Is not answered if successful.*
    """

    class Meta:
        name = "authenticate"

    password: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Cancel(AdminLobbyRequest):
    """
    Deletes the GameRoom and cancels the Game within.
    """

    class Meta:
        name = "cancel"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )


@dataclass
class JoinedGameRoom(ResponsePacket):
    """
    Sent to all administrative clients after a player joined a GameRoom via a JoinRoomRequest.
    """

    class Meta:
        name = "joinedGameRoom"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )
    player_count: Optional[int] = field(
        default=None,
        metadata={
            "name": "playerCount",
            "type": "Attribute",
        }
    )


@dataclass
class Observe(AdminLobbyRequest):
    """
    Sent to client as response to successfully joining a GameRoom as Observer.
    """

    class Meta:
        name = "observe"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )


@dataclass
class Pause(AdminLobbyRequest):
    """
    Indicates to observers that the game has been (un)paused.
    """

    class Meta:
        name = "pause"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )
    pause: Optional[bool] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Slot(ProtocolPacket):
    """
    Slots for a game which contains the player's name and its attributes.
    """

    class Meta:
        name = "slot"

    display_name: Optional[str] = field(
        default=None,
        metadata={
            "name": "displayName",
            "type": "Attribute",
        }
    )
    can_timeout: Optional[bool] = field(
        default=None,
        metadata={
            "name": "canTimeout",
            "type": "Attribute",
        }
    )
    reserved: Optional[bool] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Step(AdminLobbyRequest):
    """
    When the client is authenticated as administrator,
    it can send this step request to the server to advance the game for one move.
    This is not possible if the game is not paused.
    """

    class Meta:
        name = "step"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )


@dataclass
class Prepare(AdminLobbyRequest):
    """
    When the client is authenticated as administrator,
    it can send this request to prepare the room for the game.
    """

    class Meta:
        name = "prepare"

    game_type: Optional[str] = field(
        default=None,
        metadata={
            "name": "gameType",
            "type": "Attribute",
        }
    )
    pause: Optional[bool] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    slot: List[Slot] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class Join(LobbyRequest):
    """
    Joins any room that is open.
    If no room is open,
    a new room is created by the server.
    """

    class Meta:
        name = "join"


@dataclass
class JoinPrepared(LobbyRequest):
    """
    Join a prepared room with a reservation code.
    """

    class Meta:
        name = "joinPrepared"

    reservation_code: Optional[str] = field(
        default=None,
        metadata={
            "name": "reservationCode",
            "type": "Attribute",
        }
    )


@dataclass
class JoinRoom(LobbyRequest):
    """
    To join a room with a `room_id`.
    """

    class Meta:
        name = "joinRoom"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )


@dataclass
class Fishes:
    """
    The amount of fishes a player has.
    """

    class Meta:
        name = "fishes"

    int_value: List[int] = field(
        default_factory=list,
        metadata={
            "name": "int",
            "type": "Element",
        }
    )


@dataclass
class Fragment:
    """
    This holds the fragments of a winning definition.
    """

    class Meta:
        name = "fragment"

    name: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    aggregation: Optional[str] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    relevant_for_ranking: Optional[bool] = field(
        default=None,
        metadata={
            "name": "relevantForRanking",
            "type": "Element",
        }
    )


@dataclass
class From:
    """
    The origin of a move.
    """

    class Meta:
        name = "from"

    x: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    y: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Joined(ResponsePacket):
    """
    Sent to all clients after a player joined a GameRoom via a Join Request.
    """

    class Meta:
        name = "joined"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )


@dataclass
class ListType:
    """
    Represents a list for the game board, that contains the fields.
    """

    class Meta:
        name = "list"

    field_value: List[Union[str, int]] = field(
        default_factory=list,
        metadata={
            "name": "field",
            "type": "Element",
        }
    )


@dataclass
class Player:
    """
    The player that has won the game.
    """

    class Meta:
        name = "player"

    name: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    team: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Score:
    """
    Score of the players when the game has ended.
    """

    class Meta:
        name = "score"

    cause: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    reason: Optional[object] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    part: List[int] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class To:
    """
    The target of a move.
    """

    class Meta:
        name = "to"

    x: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    y: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Winner:
    """
    The winner of a game.
    """

    class Meta:
        name = "winner"

    team: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Board:
    """
    The protocol representation of a board.
    It contains a list of list of fields, which size is 7x7.
    """

    class Meta:
        name = "board"

    list_value: List[ListType] = field(
        default_factory=list,
        metadata={
            "name": "list",
            "type": "Element",
        }
    )


@dataclass
class Definition:
    """
    The definition of a result of a game.
    If for instance one player made an error move, the game is over and the other player wins,
    the definition will tell that the other player wins, because of the error.
    """

    class Meta:
        name = "definition"

    fragment: List[Fragment] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class Entry:
    """
    Is send when a game is won by one of the players.
    This element contains the winning player and the score of the player.
    """

    class Meta:
        name = "entry"

    player: Optional[Player] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    score: Optional[Score] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class LastMove:
    """
    Last move of a player.
    """

    class Meta:
        name = "lastMove"

    from_value: Optional[From] = field(
        default=None,
        metadata={
            "name": "from",
            "type": "Element",
        }
    )
    to: Optional[To] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class Scores:
    """
    Then result of a game when its over.
    """

    class Meta:
        name = "scores"

    entry: List[Entry] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class State(ObservableRoomMessage):
    """
    The state of the game, with the current board, score and last move.
    """

    class Meta:
        name = "state"

    class_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "class",
            "type": "Attribute",
        }
    )
    turn: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    start_team: Optional[str] = field(
        default=None,
        metadata={
            "name": "startTeam",
            "type": "Element",
        }
    )
    board: Optional[Board] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    last_move: Optional[LastMove] = field(
        default=None,
        metadata={
            "name": "lastMove",
            "type": "Element",
        }
    )
    fishes: Optional[Fishes] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class OriginalMessage:
    """
    The original message that was sent by the client.
    Is sent by the server if a error occurs.
    """

    class Meta:
        name = "originalMessage"

    class_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "class",
            "type": "Attribute",
        }
    )
    from_value: Optional[From] = field(
        default=None,
        metadata={
            "name": "from",
            "type": "Element",
        }
    )
    to: Optional[To] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class Data:
    """
    This element is sent by the server to the client to notify the client of a changing state of the game.
    It can contain a move, game state, or winning team with the reason.
    """

    class Meta:
        name = "data"

    class_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "class",
            "type": "Attribute",
        }
    )
    class_binding: Optional[object] = field(
        default=None
    )
    definition: Optional[Definition] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    scores: Optional[Scores] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    winner: Optional[Winner] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    from_value: Optional[From] = field(
        default=None,
        metadata={
            "name": "from",
            "type": "Element",
        }
    )
    to: Optional[To] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    state: Optional[State] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    color: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    message: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    original_message: Optional[OriginalMessage] = field(
        default=None,
        metadata={
            "name": "originalMessage",
            "type": "Element",
        }
    )


@dataclass
class Room(ProtocolPacket):
    """
    The root element of every room packet.
    It contains a data element when send that contains the actual data,
    that are needed for the game to work.
    """

    class Meta:
        name = "room"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        }
    )
    data: Optional[Data] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class WelcomeMessage(RoomOrchestrationMessage):
    """
    Welcome message is sent to the client when the client joins the room.
    In this message the server tells the client which team it is.
    """
    team: Team


@dataclass
class Result:
    """
    Result of a game.
    This will the server send after a game is finished.
    """
    definition: Definition
    scores: Scores
    winner: Winner


@dataclass
class Error:
    """
    This sends the server when the client sent a erroneous message.
    """
    message: str
    originalMessage: OriginalMessage


@dataclass
class Protocol:
    """
    This is the root element of the protocol.
    Even it's in here it will never be called,
    because the children of this root element have to be handelt separately.
    """

    class Meta:
        name = "protocol"

    authenticate: Optional[Authenticate] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    joined_game_room: Optional[JoinedGameRoom] = field(
        default=None,
        metadata={
            "name": "joinedGameRoom",
            "type": "Element",
        }
    )
    prepare: Optional[Prepare] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    observe: Optional[Observe] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    pause: Optional[Pause] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    step: Optional[Step] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    cancel: Optional[Cancel] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    join: Optional[Join] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    joined: Optional[Joined] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    room: List[Room] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        }

    )
