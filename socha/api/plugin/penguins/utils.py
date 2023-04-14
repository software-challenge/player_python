from typing import Union

from socha.api.plugin.penguins import game_state
from socha.api.plugin.penguins.board import Field, Move, CartesianCoordinate, TeamEnum, Penguin, HexCoordinate
from socha.api.plugin.penguins.game_state import GameState
from socha.api.protocol.protocol import Board, Data, \
    From, To, Team


def _convert_board(protocol_board: Board) -> game_state.Board:
    """
    Converts a protocol Board to a usable game board for using in the logic.
    :param protocol_board: A Board object in protocol format
    :type protocol_board: Board
    :return: A Board object in the format used by the game logic
    :rtype: penguins.Board
    """
    if not isinstance(protocol_board, Board):
        raise TypeError("The input must be a Board object in protocol format")

    def create_field(y: int, x: int, fields_value: Union[int, str]) -> Field:
        coordinate = CartesianCoordinate(x, y).to_hex()
        if isinstance(fields_value, int):
            return Field(coordinate, penguin=None, fish=fields_value)
        elif fields_value == "ONE":
            return Field(coordinate, penguin=Penguin(coordinate, TeamEnum.ONE), fish=0)
        elif fields_value == "TWO":
            return Field(coordinate, penguin=Penguin(coordinate, TeamEnum.TWO), fish=0)
        else:
            raise ValueError(f"Invalid field value {fields_value} at coordinates {coordinate}")

    board_list = [[create_field(y, x, fields_value) for x, fields_value in enumerate(row.field_value)] for y, row in
                  enumerate(protocol_board.list_value)]
    return game_state.Board(board_list)


def handle_move(move_response) -> Data:
    from_pos = None
    to_pos = To(x=move_response.to_value.x, y=move_response.to_value.y)
    if move_response.from_value:
        from_pos = From(x=move_response.from_value.x, y=move_response.from_value.y)
    return Data(class_value="move", from_value=from_pos, to=to_pos)


def if_last_game_state(message, last_game_state: GameState) -> GameState:
    from_value = None if not message.data.class_binding.last_move.from_value else HexCoordinate(
        x=message.data.class_binding.last_move.from_value.x,
        y=message.data.class_binding.last_move.from_value.y)
    to_value = HexCoordinate(x=message.data.class_binding.last_move.to.x,
                             y=message.data.class_binding.last_move.to.y)
    last_move = Move(team_enum=last_game_state.current_team.name,
                     from_value=from_value,
                     to_value=to_value)
    return last_game_state.perform_move(last_move)


def if_not_last_game_state(message) -> GameState:
    first_team = Team(TeamEnum.ONE,
                      fish=0,
                      penguins=[],
                      moves=[])
    second_team = Team(TeamEnum.TWO,
                       0,
                       penguins=[],
                       moves=[])
    first_team.opponent = second_team
    second_team.opponent = first_team

    return GameState(
        board=_convert_board(message.data.class_binding.board),
        turn=message.data.class_binding.turn,
        first_team=first_team,
        second_team=second_team,
        last_move=None,
    )
