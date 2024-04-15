from typing import List

from socha import _socha
import socha
from socha._socha import CubeCoordinates, Field, FieldType, GameState, Move, TeamEnum
from socha.api.protocol.protocol import (
    Acceleration,
    Actions,
    Advance,
    Board,
    Data,
    Goal,
    Island,
    LastMove,
    Passenger,
    Push,
    Sandbank,
    Turn,
    Water,
)


def _convert_board(protocol_board: Board) -> _socha.Board:
    """
    Converts a protocol Board to a usable game board for using in the logic.
    :param protocol_board: A Board object in protocol format
    :type protocol_board: Board
    :return: A Board object in the format used by the game logic
    :rtype: penguins.Board
    """
    con_segments: List[_socha.Segment] = []
    for segment in protocol_board.segment:
        con_fields: List[List[_socha.Field]] = []
        for field_array in segment.field_array:
            con_row: List[_socha.Field] = []
            for field in field_array.field:
                if isinstance(field, Water):
                    con_row.append(Field(FieldType.Water, None))
                elif isinstance(field, Sandbank):
                    con_row.append(Field(FieldType.Sandbank, None))
                elif isinstance(field, Island):
                    con_row.append(Field(FieldType.Island, None))
                elif isinstance(field, Passenger):
                    con_row.append(
                        Field(
                            FieldType.Passenger,
                            _socha.Passenger(
                                direction_from_string(
                                    field.direction), field.passenger
                            ),
                        )
                    )
                elif isinstance(field, Goal):
                    con_row.append(Field(FieldType.Goal, None))
            con_fields.append(con_row)
        con_center: _socha.CubeCoordinates = CubeCoordinates(
            q=segment.center.q, r=segment.center.r
        )
        con_segments.append(
            _socha.Segment(
                direction=direction_from_string(segment.direction),
                center=con_center,
                fields=con_fields,
            )
        )
    return _socha.Board(
        segments=con_segments,
        next_direction=direction_from_string(protocol_board.next_direction),
    )


def direction_from_string(cube_direction: str) -> _socha.CubeDirection:
    if cube_direction == "RIGHT":
        return _socha.CubeDirection.Right
    elif cube_direction == "DOWN_RIGHT":
        return _socha.CubeDirection.DownRight
    elif cube_direction == "DOWN_LEFT":
        return _socha.CubeDirection.DownLeft
    elif cube_direction == "LEFT":
        return _socha.CubeDirection.Left
    elif cube_direction == "UP_LEFT":
        return _socha.CubeDirection.UpLeft
    elif cube_direction == "UP_RIGHT":
        return _socha.CubeDirection.UpRight
    raise ValueError("Invalid cube direction")


def direction_to_string(cube_direction: _socha.CubeDirection) -> str:
    if cube_direction == _socha.CubeDirection.Right:
        return "RIGHT"
    elif cube_direction == _socha.CubeDirection.DownRight:
        return "DOWN_RIGHT"
    elif cube_direction == _socha.CubeDirection.DownLeft:
        return "DOWN_LEFT"
    elif cube_direction == _socha.CubeDirection.Left:
        return "LEFT"
    elif cube_direction == _socha.CubeDirection.UpLeft:
        return "UP_LEFT"
    elif cube_direction == _socha.CubeDirection.UpRight:
        return "UP_RIGHT"
    raise ValueError("Invalid cube direction")


def handle_move(move_response):
    actions = move_response.actions
    protocol_actions = [
        (
            Acceleration(acc=a.acc)
            if isinstance(a, _socha.Accelerate)
            else (
                Advance(distance=a.distance)
                if isinstance(a, _socha.Advance)
                else (
                    Push(direction=direction_to_string(a.direction))
                    if isinstance(a, _socha.Push)
                    else Turn(direction=direction_to_string(a.direction))
                )
            )
        )
        for a in actions
    ]
    return Data(class_value="move", actions=Actions(actions=protocol_actions))


def _merge_advances(actions):
    """
    Merges consecutive Advance actions into a single action by adding their distances.
    This is a workaround for handling multiple Advance actions in a sequence.

    Args:
        actions (list): A list of actions.

    Returns:
        list: A new list of actions where consecutive Advance actions have been merged.

    Note:
        This function modifies the input list 'actions' in-place.
    """
    new_actions = []
    for i in range(len(actions) - 1):
        if isinstance(actions[i], _socha.Advance) and isinstance(
            actions[i + 1], _socha.Advance
        ):
            actions[i + 1].distance += actions[i].distance
            actions[i] = None
    new_actions = [a for a in actions if a is not None]
    return new_actions


def convert_to_moves(last_move: LastMove | None) -> Move:
    if last_move is None:
        return None

    actions = []

    for action in last_move.actions.actions:
        if isinstance(action, Acceleration):
            actions.append(socha._socha.Accelerate(action.acc))
        elif isinstance(action, Advance):
            actions.append(socha._socha.Advance(action.distance))
        elif isinstance(action, Push):
            actions.append(socha._socha.Push(
                direction_from_string(action.direction)))
        elif isinstance(action, Turn):
            actions.append(socha._socha.Turn(
                direction_from_string(action.direction)))

    return Move(actions)


def message_to_state(message) -> GameState:
    """
    Constructs a GameState from the provided message, ensuring to reflect the
    current state based on the ships' positions, teams, and other attributes.

    Args:
        message: The input message containing the current game state.

    Returns:
        GameState: The constructed game state from the message.
    """

    def create_ship(ship_data, team_enum_value) -> _socha.Ship:
        """Helper function to create a ship from the ship data."""
        position = CubeCoordinates(
            q=ship_data.position.q, r=ship_data.position.r)
        team = TeamEnum.One if team_enum_value == "ONE" else TeamEnum.Two
        return _socha.Ship(
            position=position,
            team=team,
            coal=ship_data.coal,
            passengers=ship_data.passengers,
            points=ship_data.points,
            speed=ship_data.speed,
            free_turns=ship_data.free_turns,
            direction=direction_from_string(ship_data.direction),
        )

    first_ship_data, second_ship_data = message.data.class_binding.ship
    first_ship = create_ship(first_ship_data, first_ship_data.team)
    second_ship = create_ship(second_ship_data, second_ship_data.team)

    current_team_enum = (
        TeamEnum.One
        if message.data.class_binding.current_team == "ONE"
        else TeamEnum.Two
    )

    return GameState(
        board=_convert_board(message.data.class_binding.board),
        turn=message.data.class_binding.turn,
        current_ship=first_ship if current_team_enum == TeamEnum.One else second_ship,
        other_ship=second_ship if current_team_enum == TeamEnum.One else first_ship,
        last_move=convert_to_moves(message.data.class_binding.last_move),
    )
