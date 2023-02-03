# `GameState`
Die Signatur hat sich geändert:
  - `def __init__(self, board: Board, turn: int, start_team: Team, fishes: Fishes, last_move: Move = None)`
  - `def __init__(self, board: Board, turn: int, first_team: Team, second_team: Team, last_move: Optional[Move])`
## `fishes`
Das Attribut wurde entfernt. Die Anzahl an Fischen eines Teams kann nun in dem Team-Objekt selbst gefunden werden:
    - z.B.: `self.game_state.first_team.fish`

# `TeamEnum`
Diese Klasse ist lediglich eine Enumeration der beiden Strings `ONE` und `TWO`. Sie dient den folgenden Klassen, 
das Team zu unterscheiden und das Handhaben mit den Strings eleganter zu gestalten.
Die Enumaration kann folgendermaßen benutzt werden:
  - `TeamEnum.ONE`, um das erste Team darzustellen,
  - `TeamEnum.TWO`, um das zweite Team darzustellen.

# `Move`
Die Signatur hat sich geändert:
  - `def __init__(self, to_value: HexCoordinate, from_value: HexCoordinate = None)`
  - `def __init__(self, team_enum: TeamEnum, to_value: HexCoordinate, from_value: Optional[HexCoordinate])`

# `Penguin`
Diese Klasse wird nun genutzt, um ein Pinguin zu repräsentieren:
  - `def __init__(self, coordinate: HexCoordinate, team_enum: TeamEnum)`
Diese Klasse wird z.B. in der `Field`-Klasse genutzt, um anzuzeigen, dass sich dort ein Pinguin befindet.

# `Team`
Die Signatur hat sich geändert:
  - `def __init__(self, color: str)`
  - `def __init__(self, name: TeamEnum, fish: int, penguins: List[Penguin], moves: List[Move])`
Die `opponent()`-Methode wurde entfernt und befindet sich nun in der `GameState`-Klasse.

Dies dient dazu, die Struktur und Relationen der Klassen sinnvoller zu gestalten.

# `Board`
Hier hat sich äußerlich kaum etwas geändert. Allerdings wurden Methoden entsprechend der neuen Struktur angepasst.
So hat sich zum Beispiel folgende Methode geändert:
  - `def get_teams_penguins(self, team: Team) -> List[HexCoordinate]`
  - `def get_teams_penguins(self, team: TeamEnum) -> List[Penguin]:`
Das bedeutet, dass nun diese Methode z.B. so aufgerufen werden muss: `self.board.get_teams_penguin(TeamEnum.ONE)`. Angenommen es besteht eine Referenz zu einem Board-Objekt mit dem Namen `board` als Attribut einer Klasse.
