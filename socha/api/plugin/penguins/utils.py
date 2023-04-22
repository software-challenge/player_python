from socha.api.networking.xml_protocol_interface import XMLProtocolInterface
from socha.api.plugin.penguins import game_state
from socha.api.plugin.penguins.board import Field, Move, CartesianCoordinate, TeamEnum, Penguin, HexCoordinate
from socha.api.plugin.penguins.game_state import GameState
from socha.api.protocol.protocol import State, Board, Data, \
    Error, From, Join, Joined, JoinPrepared, JoinRoom, To, Team, Room, Result, MoveRequest, Left, Errorpacket
from socha.api.protocol.protocol_packet import ProtocolPacket


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

    board_list = []
    for y, row in enumerate(protocol_board.list_value):
        board_list.append([])
        for x, fields_value in enumerate(row.field_value):
            coordinate = CartesianCoordinate(x, y).to_hex()
            if type(fields_value) is int:
                board_list[y].append(Field(coordinate, penguin=None, fish=fields_value))
            elif fields_value == "ONE":
                board_list[y].append(Field(coordinate, penguin=Penguin(coordinate, TeamEnum.ONE), fish=0))
            elif fields_value == "TWO":
                board_list[y].append(Field(coordinate, penguin=Penguin(coordinate, TeamEnum.TWO), fish=0))
            else:
                raise ValueError(f"Invalid field value {fields_value} at coordinates {coordinate}")

    return game_state.Board(board_list)


def handle_move(move_response) -> Data:
    from_pos = None
    to_pos = To(x=move_response.to_value.x, y=move_response.to_value.y)
    if move_response.from_value:
        from_pos = From(x=move_response.from_value.x, y=move_response.from_value.y)
    return Data(class_value="move", from_value=from_pos, to=to_pos)
