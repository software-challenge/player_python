## <a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a> Python Client der Software-Challenge Germany 2023!

In diesem Repository befindet sich das Python Package der [Software-Challenge](https://www.software-challenge.de), ein
Programmierwettbewerb für Schüler. Dabei wird für ein jährlich wechselndes Spiel eine künstliche Intelligenz entwickelt,
die sich dann in Duellen gegen andere durchsetzen muss.

Dieses Jahr ist es das Spiel **Hey, danke für den Fisch!**.

### Installation

...

### How To Use It

````python
from src.SoftwareChallengeClient.player.Starter import Starter
from src.SoftwareChallengeClient.api.networking.clients.PlayerClient import IClientHandler
from src.SoftwareChallengeClient.api.sc.Plugin2023 import GameState, Move


class Logic(IClientHandler):
    gameState: GameState

    def calculateMove(self) -> Move:
        ...

    def onUpdate(self, state: GameState):
        self.gameState = state


if __name__ == "__main__":
    Starter("Localhost", 13050, Logic())
````