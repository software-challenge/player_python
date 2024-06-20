from typing import List
from socha import _socha
from socha.api.protocol.protocol import (
    Board,
    Cards,
    Room,
    ScPlugin2025Hare,
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

    for field in protocol_board.fields:
        if field == 'START':
            track.append(_socha.Field.Start)
        elif field == 'MARKET':
            track.append(_socha.Field.Market)
        elif field == 'HARE':
            track.append(_socha.Field.Hare)
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

    return _socha.Board(track=track)


def map_xml_to_card(cards: Cards) -> List[_socha.Card]:
    return_cards: List[_socha.Card] = []
    for card in cards.card:
        if card == 'EAT_SALAD':
            return_cards.append(_socha.Card.EatSalad)
        elif card == 'HURRY_AHEAD':
            return_cards.append(_socha.Card.HurryAhead)
        elif card == 'FALL_BACK':
            return_cards.append(_socha.Card.FallBack)
        elif card == 'SWAP_CARROTS':
            return_cards.append(_socha.Card.SwapCarrots)
    return return_cards


def map_card_to_xml(cards: List[_socha.Card]) -> None | Cards:
    if not cards:
        return None
    return_cards: Cards = Cards()
    for card in cards:
        if card == _socha.Card.EatSalad:
            return_cards.sc_plugin2025_card.append('EAT_SALAD')
        elif card == _socha.Card.FallBack:
            return_cards.sc_plugin2025_card.append('FALL_BACK')
        elif card == _socha.Card.HurryAhead:
            return_cards.sc_plugin2025_card.append('HURRY_AHEAD')
        elif card == _socha.Card.SwapCarrots:
            return_cards.sc_plugin2025_card.append('SWAP_CARROTS')
    return return_cards


def handle_move(move_response: _socha.Move):
    if isinstance(move_response.action, _socha.Advance):
        advance: _socha.Advance = move_response.action
        return Data(
            class_value='advance',
            distance=advance.distance,
            cards=map_card_to_xml(advance.cards),
        )
    elif isinstance(move_response.action, _socha.EatSalad):
        return Data(class_value='eatsalad')
    elif isinstance(move_response.action, _socha.ExchangeCarrots):
        exchangeCarrots: _socha.ExchangeCarrots = move_response.action
        return Data(class_value='exchangecarrots', value=exchangeCarrots.amount)
    elif isinstance(move_response.action, _socha.FallBack):
        return Data(class_value='fallback')


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

    def create_hare(hare: ScPlugin2025Hare) -> _socha.Hare:
        return _socha.Hare(
            cards=map_xml_to_card(cards=hare.cards),
            carrots=hare.carrots,
            position=hare.position,
            salad_eaten=hare.salad_eaten,
            salads=hare.salads,
            team=_socha.TeamEnum.One if hare.team == 'ONE' else _socha.TeamEnum.Two,
        )

    return _socha.GameState(
        board=map_board(state.board),
        player_one=create_hare(state.sc_plugin2025_hare[0]),
        player_two=create_hare(state.sc_plugin2025_hare[1]),
        turn=state.turn,
    )
