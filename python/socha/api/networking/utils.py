import re
from typing import List
from socha import _socha
from socha.api.protocol.protocol import (
    Coordinate,
    Board,
    Room,
    State,
    Data,
)


def map_board(protocol_board: Board) -> _socha.Board:
    """
    Converts a protocol Board to a usable game board for using in the logic.
    :param protocol_board: A Board object in protocol format
    :type protocol_board: Board
    :return: A Board object in the format used by the game logic
    :rtype: _socha.Board
    """
    
    board_map: List[List[_socha.FieldType]] = []

    for row in protocol_board.rows:
        board_map.append([])
        for field in row.field_value:
            if field == 'EMPTY':
                board_map[-1].append(_socha.FieldType.Empty)
            elif field == 'ONE_S':
                board_map[-1].append(_socha.FieldType.OneS)
            elif field == 'ONE_M':
                board_map[-1].append(_socha.FieldType.OneM)
            elif field == 'ONE_L':
                board_map[-1].append(_socha.FieldType.OneL)
            elif field == 'TWO_S':
                board_map[-1].append(_socha.FieldType.TwoS)
            elif field == 'TWO_M':
                board_map[-1].append(_socha.FieldType.TwoM)
            elif field == 'TWO_L':
                board_map[-1].append(_socha.FieldType.TwoL)
            elif field == 'SQUID':
                board_map[-1].append(_socha.FieldType.Squid)
            else:
                raise ValueError(f'Unknown field type: {field}')
    
    return _socha.Board(map=board_map)

def map_string_to_direction(direction: str) -> _socha.Direction:
    direction = re.sub(r'[^A-Za-z0-9_]', '', direction)

    if direction == 'UP':
        return _socha.Direction.Up
    elif direction == 'UP_RIGHT':
        return _socha.Direction.UpRight
    elif direction == 'RIGHT':
        return _socha.Direction.Right
    elif direction == 'DOWN_RIGHT':
        return _socha.Direction.DownRight
    elif direction == 'DOWN':
        return _socha.Direction.Down
    elif direction == 'DOWN_LEFT':
        return _socha.Direction.DownLeft
    elif direction == 'LEFT':
        return _socha.Direction.Left
    elif direction == 'UP_LEFT':
        return _socha.Direction.UpLeft
    else:
        raise ValueError(f'Unknown direction: {direction}')

def map_direction_to_string(direction: _socha.Direction):

    if direction == _socha.Direction.Up:
        return 'UP'
    elif direction == _socha.Direction.UpRight:
        return 'UP_RIGHT'
    elif direction == _socha.Direction.Right:
        return 'RIGHT'
    elif direction == _socha.Direction.DownRight:
        return 'DOWN_RIGHT'
    elif direction == _socha.Direction.Down:
        return 'DOWN'
    elif direction == _socha.Direction.DownLeft:
        return 'DOWN_LEFT'
    elif direction == _socha.Direction.Left:
        return 'LEFT'
    elif direction == _socha.Direction.UpLeft:
        return 'UP_LEFT'
    else:
        raise ValueError(f'Unknown direction: {direction}')

def handle_move(move_response: _socha.Move) -> Data:

    return Data(
            class_value='move',
            from_=Coordinate(move_response.start.x, move_response.start.y), # invert y coordinate for server compatibility
            direction=map_direction_to_string(move_response.direction),
        )

def message_to_state(message: Room) -> _socha.GameState:
    """
    Constructs a GameState from the provided message, ensuring to reflect the
    current state based on the ships' positions, teams, and other attributes.

    Args:
        message: The input message containing the current game state.
        second_last_move: the last_move object from the last game state before this game state

    Returns:
        GameState: The constructed game state from the message.
    """

    state: State = message.data.class_binding

    # extract last move of current gameState
    state_last_move = state.last_move.class_binding if state.last_move and state.last_move.class_binding else None

    return _socha.GameState(
        board=map_board(state.board),
        turn=state.turn,
        last_move=state_last_move,
    )
