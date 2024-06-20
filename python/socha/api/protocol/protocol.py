from dataclasses import dataclass, field
from typing import List, Optional

from socha._socha import TeamEnum

from socha.api.protocol.protocol_packet import AdminLobbyRequest, LobbyRequest, ProtocolPacket, ResponsePacket
from socha.api.protocol.room_message import ObservableRoomMessage, RoomMessage, RoomOrchestrationMessage


@dataclass
class Cards:
    class Meta:
        name = "cards"

    card: List[str] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "min_occurs": 1,
        },
    )


@dataclass
class Board:
    class Meta:
        name = "board"

    fields: List[str] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "min_occurs": 1,
        },
    )
    size: Optional[int] = field(
        default=None,
        metadata={
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class ScPlugin2025Hare:
    class Meta:
        name = "sc.plugin2025.Hare"

    team: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        },
    )
    position: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        },
    )
    salads: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        },
    )
    carrots: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        },
    )
    salad_eaten: Optional[bool] = field(
        default=None,
        metadata={
            "name": "saladEaten",
            "type": "Attribute",
            "required": True,
        },
    )
    cards: Optional[Cards] = field(
        default=None,
        metadata={
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class Player:
    class Meta:
        name = "player"

    team: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )


@dataclass
class State(ObservableRoomMessage):
    class Meta:
        name = "state"

    class_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "class",
            "type": "Attribute",
            "required": True,
        },
    )
    start_team: Optional[str] = field(
        default=None,
        metadata={
            "name": "startTeam",
            "type": "Attribute",
            "required": True,
        },
    )
    turn: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        },
    )
    board: Optional[Board] = field(
        default=None,
        metadata={
            "type": "Element",
            "required": True,
        },
    )
    sc_plugin2025_hare: List[ScPlugin2025Hare] = field(
        default_factory=list,
        metadata={
            "name": "sc.plugin2025.Hare",
            "type": "Element",
            "min_occurs": 1,
        },
    )


@dataclass
class OriginalRequest(ProtocolPacket):
    class Meta:
        name = "originalRequest"

    class_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "class",
            "type": "Attribute",
        },
    )
    reservation_code: Optional[str] = field(
        default=None,
        metadata={
            "name": "reservationCode",
            "type": "Attribute",
        },
    )


@dataclass
class Errorpacket(ProtocolPacket):
    class Meta:
        name = "errorpacket"

    message: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
    )
    original_request: Optional[OriginalRequest] = field(
        default=None,
        metadata={
            "name": "originalRequest",
            "type": "Element",
        },
    )


@dataclass
class Left(ProtocolPacket):
    """
    If the game is over the server will _send this message to the clients and closes the connection afterward.
    """

    class Meta:
        name = "left"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
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
        name = "close"


@dataclass
class Authenticate(AdminLobbyRequest):
    """
    Authenticates a client as administrator to _send AdminLobbyRequest`s.
    *Is not answered if successful.*
    """

    class Meta:
        name = "authenticate"

    password: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
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
        },
    )


@dataclass
class JoinedGameRoom(ObservableRoomMessage):
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
        },
    )
    player_count: Optional[int] = field(
        default=None,
        metadata={
            "name": "playerCount",
            "type": "Attribute",
        },
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
        },
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
        },
    )
    pause: Optional[bool] = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
    )


@dataclass
class Slot(RoomOrchestrationMessage):
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
        },
    )
    can_timeout: Optional[bool] = field(
        default=None,
        metadata={
            "name": "canTimeout",
            "type": "Attribute",
        },
    )
    reserved: Optional[bool] = field(
        default=None,
        metadata={
            "type": "Attribute",
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
        name = "step"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        },
    )


@dataclass
class Prepare(RoomOrchestrationMessage):
    """
    When the client is authenticated as administrator,
    it can _send this request to prepare the room for the game.
    """

    class Meta:
        name = "prepare"

    game_type: Optional[str] = field(
        default=None,
        metadata={
            "name": "gameType",
            "type": "Attribute",
        },
    )
    pause: Optional[bool] = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
    )
    slot: List[Slot] = field(
        default_factory=list,
        metadata={
            "type": "Element",
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
        },
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
        },
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
        },
    )
    aggregation: Optional[str] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    relevant_for_ranking: Optional[bool] = field(
        default=None,
        metadata={
            "name": "relevantForRanking",
            "type": "Element",
        },
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
        },
    )


@dataclass
class Score:
    """
    Score of the players when the game has ended.
    """

    class Meta:
        name = "score"

    part: List[int] = field(
        default_factory=list,
        metadata={
            "type": "Element",
            "min_occurs": 1,
        },
    )


@dataclass
class Winner:
    class Meta:
        name = "winner"

    team: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        },
    )
    regular: Optional[bool] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
        },
    )
    reason: Optional[str] = field(
        default=None,
        metadata={
            "type": "Attribute",
            "required": True,
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
        name = "definition"

    fragment: List[Fragment] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        },
    )


@dataclass
class Entry:
    """
    Is _send when a game is won by one of the players.
    This element contains the winning player and the score of the player.
    """

    class Meta:
        name = "entry"

    player: Optional[Player] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    score: Optional[Score] = field(
        default=None,
        metadata={
            "type": "Element",
        },
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
    """

    class Meta:
        name = "originalMessage"

    class_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "class",
            "type": "Attribute",
            "required": True,
        }
    )
    distance: Optional[int] = field(
        default=None,
        metadata={
            "type": "Element",
            "required": True,
        }
    )
    cards: Optional[Cards] = field(
        default=None,
        metadata={
            "type": "Element",
        }
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
        name = "data"

    class_value: Optional[str] = field(
        default=None,
        metadata={
            "name": "class",
            "type": "Attribute",
            "required": True,
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
    original_message: Optional[OriginalMessage] = field(
        default=None,
        metadata={
            "name": "originalMessage",
            "type": "Element",
        },
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
    state: Optional[State] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    distance: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )
    cards: Optional[Cards] = field(
        default=None,
        metadata={
            "type": "Element",
        }
    )
    amount: Optional[int] = field(
        default=None,
        metadata={
            "type": "Attribute",
        }
    )


@dataclass
class Room(ProtocolPacket):
    class Meta:
        name = "room"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
            "required": True,
        },
    )
    data: Optional[Data] = field(
        default=None,
        metadata={
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class Observed(RoomOrchestrationMessage):
    class Meta:
        name = "observed"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        },
    )


@dataclass
class Prepared(RoomOrchestrationMessage):
    class Meta:
        name = "prepared"

    room_id: Optional[str] = field(
        default=None,
        metadata={
            "name": "roomId",
            "type": "Attribute",
        },
    )
    reservation: List[str] = field(
        default_factory=list,
        metadata={
            "type": "Element",
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
        name = "protocol"

    authenticate: Optional[Authenticate] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    joined_game_room: Optional[JoinedGameRoom] = field(
        default=None,
        metadata={
            "name": "joinedGameRoom",
            "type": "Element",
        },
    )
    prepare: Optional[Prepare] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    observe: Optional[Observe] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    pause: Optional[Pause] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    step: Optional[Step] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    cancel: Optional[Cancel] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    join: Optional[Join] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    joined: Optional[Joined] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    room: List[Room] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        },
    )
    prepared: Optional[Prepared] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
    observed: Optional[Observed] = field(
        default=None,
        metadata={
            "type": "Element",
        },
    )
