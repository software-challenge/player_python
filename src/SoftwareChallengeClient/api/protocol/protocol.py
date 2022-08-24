from dataclasses import dataclass, field
from typing import List, Optional, Union

from src.SoftwareChallengeClient.api.protocol import AdminLobbyRequest, ResponsePacket, ProtocolPacket, LobbyRequest
from src.SoftwareChallengeClient.api.protocol.room.IRoomMessage import RoomOrchestrationMessage
from src.SoftwareChallengeClient.api.sc.Plugin2023 import Team


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
    class Meta:
        name = "join"


@dataclass
class JoinPrepared(LobbyRequest):
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
    class Meta:
        name = "scores"

    entry: List[Entry] = field(
        default_factory=list,
        metadata={
            "type": "Element",
        }
    )


@dataclass
class State:
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


@dataclass
class Room(ProtocolPacket):
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
    team: Team


@dataclass
class Result:
    definition: Definition
    scores: Scores
    winner: Winner


@dataclass
class Protocol:
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
