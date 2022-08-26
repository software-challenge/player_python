## <a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a>

# Python Client for the Software-Challenge Germany 2023

> Please note that this is a very early version, which may still contain some bugs.
> However, the client is able to play a game from start to end.
> 
> If you have any questions about this package, you can write a issue 
> or for faster answer write a message on our [Discord](https://discord.gg/ARZamDptG5) server.
>
> If you find bugs,
> or have suggestions for improvements,
> please post an issue,
> or contribute to the project yourself.
>
> Thanks a lot!

This repository contains the Python package for the
[Software-Challenge Germany](https://www.software-challenge.de), a programming competition for students. The students
have to develop an artificial intelligence that plays and competes against other opponents in an annually changing game.

> This year it is the game
> **[Hey, danke für den Fisch!](https://docs.software-challenge.de/spiele/penguins)**.

## Installation

The installation is quite simple with pip.

```commandline
pip install socha
```

If you want to install the package manually, then you have to download the release of your choice, unpack the package
and then run `setup.py` with Python.

```commandline
python --user setup.py install
```

This should satisfy the dependencies and you can start right away.

## Getting Started

If you want to start with the Software-Challenge Python client, you have to import some dependencies first.

- Your logic must inherit from the `IClientHandler` in order for it to communicate correctly with the API.
- Furthermore, you should import the plugin of this year's game so that you can communicate with the `GameState`
  and use other functionalities.
- To make your player start when the script is executed, you have to import the `Starter` and call it later.

````python
from socha.api.networking.PlayerClient import IClientHandler
from socha.api.plugin.Penguins import *
from socha.Starter import Starter
````

If you now want to develop and implement your logic, then the structure of the class should look like this.

````python
class Logic(IClientHandler):
  gameState: GameState

  def calculate_move(self) -> Move:
    possibleMoves = self.gameState.get_possible_moves()
    return possibleMoves[0]

  def on_update(self, state: GameState):
    self.gameState = state

  def on_error(self, logMessage: str):
    ...
````

The above example is the simplest working Logic you can build. As you can see the Logic must inherit from
the `IClientHandler`, so that you can overwrite its methods and the api knows where to find your logic.

If you're done with your version of an working player, than you have to finish your file with this function, where you
call the Starter with your desired arguments.

````python
if __name__ == "__main__":
    Starter("localhost", 13050, Logic())
````