## <a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a> Python Client of the Software-Challenge Germany 2023

This repository contains the Python package for the
[Software-Challenge Germany](https://www.software-challenge.de), a programming competition for students. The students
have to develop an artificial intelligence that plays and competes against other opponents in an annually changing game.

> This year it is the game
> **[Hey, danke für den Fisch!](https://docs.software-challenge.de/spiele/penguins)**.

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