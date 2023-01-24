# `GameState`


- Die Signatur hat sich geändert:
  - `def __init__(self, board: Board, turn: int, start_team: Team, fishes: Fishes, last_move: Move = None)`
  - `def __init__(self, board: Board, turn: int, first_team: Team, second_team: Team, last_move: Optional[Move])`
## `fishes`
  - Das Attribut wurde entfernt. Die Anzahl an Fischen eines Teams kann nun in dem Team-Objekt selbst gefunden werden:
    - z.B.: `self.game_state.first_team.fish`

# `TeamEnum`
Diese Klasse ist lediglich eine Enumeration der beiden Strings `ONE` und `TWO`. Sie dient den folgenden Klassen, 
das Team zu unterscheiden und das Handhaben mit den Strings eleganter zu gestalten.

# `Move`

- Die Signatur hat sich geändert:
  - `def __init__(self, to_value: HexCoordinate, from_value: HexCoordinate = None)`
  - `def __init__(self, team_enum: TeamEnum, to_value: HexCoordinate, from_value: Optional[HexCoordinate])`

# `Team`

- Die Signatur hat sich geändert:
  - `def __init__(self, color: str)`
  - `def __init__(self, name: TeamEnum, fish: int, penguins: List[Penguin], moves: List[Move])`

Dies dient dazu, die Struktur und Relationen der Klassen sinnvoller zu gestalten.