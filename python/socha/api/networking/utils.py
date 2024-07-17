import re
from typing import List
from socha import _socha
from socha.api.protocol.protocol import (
    Board,
    Room,
    Hare,
    State,
    Data,
)


def map_board(protocol_board: Board) -> _socha.Board:
    """
    Converts a protocol Board to a usable game board for using in the logic.
    :param protocol_board: A Board object in protocol format
    :type protocol_board: Board
    :return: A Board object in the format used by the game logic
    :rtype: penguins.Board
    """
    track: List[_socha.Field] = []

    for field in protocol_board.field_value:
        if field == 'START':
            track.append(_socha.Field.Start)
        elif field == 'MARKET':
            track.append(_socha.Field.Market)
        elif field == 'HARE':
            track.append(_socha.Field.Hare)
        elif field == 'HEDGEHOG':
            track.append(_socha.Field.Hedgehog)
        elif field == 'CARROTS':
            track.append(_socha.Field.Carrots)
        elif field == 'POSITION_1':
            track.append(_socha.Field.Position1)
        elif field == 'POSITION_2':
            track.append(_socha.Field.Position2)
        elif field == 'SALAD':
            track.append(_socha.Field.Salad)
        elif field == 'GOAL':
            track.append(_socha.Field.Goal)
        else:
            raise ValueError(f'Unknown field type: {field}')

    return _socha.Board(track=track)


def map_card_to_string(card: _socha.Card) -> str:
    if card == _socha.Card.EatSalad:
        return 'EAT_SALAD'
    elif card == _socha.Card.HurryAhead:
        return 'HURRY_AHEAD'
    elif card == _socha.Card.FallBack:
        return 'FALL_BACK'
    elif card == _socha.Card.SwapCarrots:
        return 'SWAP_CARROTS'
    else:
        raise ValueError(f'Unknown card type: {card}')


def map_string_to_card(card: str) -> _socha.Card:
    card = re.sub(r'[^A-Za-z0-9_]', '', card)

    if card == 'EAT_SALAD':
        return _socha.Card.EatSalad
    elif card == 'HURRY_AHEAD':
        return _socha.Card.HurryAhead
    elif card == 'FALL_BACK':
        return _socha.Card.FallBack
    elif card == 'SWAP_CARROTS':
        return _socha.Card.SwapCarrots
    else:
        raise ValueError(f'Unknown card type: {card}')


def handle_move(move_response: _socha.Move) -> Data:
    if isinstance(move_response.action, _socha.Advance):
        advance: _socha.Advance = move_response.action
        return Data(
            class_value='advance',
            distance=advance.distance,
            card=[map_card_to_string(card) for card in advance.cards],
        )
    elif isinstance(move_response.action, _socha.EatSalad):
        return Data(class_value='eatsalad')
    elif isinstance(move_response.action, _socha.ExchangeCarrots):
        exchangeCarrots: _socha.ExchangeCarrots = move_response.action
        return Data(class_value='exchangecarrots', amount=exchangeCarrots.amount)
    elif isinstance(move_response.action, _socha.FallBack):
        return Data(class_value='fallback')
    else:
        raise ValueError(f'Unknown move response action: {move_response.action}')


def message_to_state(message: Room) -> _socha.GameState:
    """
    Constructs a GameState from the provided message, ensuring to reflect the
    current state based on the ships' positions, teams, and other attributes.

    Args:
        message: The input message containing the current game state.

    Returns:
        GameState: The constructed game state from the message.
    """
    state: State = message.data.class_binding

    def create_hare(hare: Hare) -> _socha.Hare:
        return _socha.Hare(
            cards=[map_string_to_card(card) for card in hare.cards.card]
            if hare.cards
            else [],
            carrots=hare.carrots,
            position=hare.position,
            last_move=_socha.Move(action=hare.last_action.class_binding)
            if hare.last_action and hare.last_action.class_binding
            else None,
            salads=hare.salads,
            team=_socha.TeamEnum.One if hare.team == 'ONE' else _socha.TeamEnum.Two,
        )

    return _socha.GameState(
        board=map_board(state.board),
        player_one=create_hare(state.hare[0]),
        player_two=create_hare(state.hare[1]),
        turn=state.turn,
    )
