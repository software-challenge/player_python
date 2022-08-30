## <a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a>

# Python Client for the Software-Challenge Germany 2023
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/FalconsSky/Software-Challenge-Python-Client/static%20and%20unit%20tests?label=Test)](https://github.com/FalconsSky/Software-Challenge-Python-Client)
[![Read the Docs](https://img.shields.io/readthedocs/software-challenge-python-client?label=Docs)](https://software-challenge-python-client.readthedocs.io/en/master)
[![PyPI](https://img.shields.io/pypi/v/socha?label=PyPi)](https://pypi.org/project/socha/)
[![Discord](https://img.shields.io/discord/233577109363097601?color=blue&label=Discord)](https://discord.gg/ARZamDptG5)
[![Documentation](https://img.shields.io/badge/Software--Challenge%20-Documentation-%234299e1)](https://docs.software-challenge.de/)
[![Website](https://img.shields.io/badge/Software--Challenge-Website-%23D9994F)](https://software-challenge.de/)
> Please note that this is a very early version, which may still contain some bugs. However, the client is able to play a game from start to end.

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
python setup.py install --user
```

This should satisfy the dependencies and you can start right away.

## Getting Started

If you want to start with the Software-Challenge Python Client, you have to import some dependencies first.

The import is kept very simple,
since herewith all needed dependencies are imported,
due to changes of the `__init__.py`.
But if you want to avoid unnecessary imports,
you can of course import only what you actually need.

````python
from socha import *
````

If you now want to develop and implement your logic, then the structure of the class should look like this.

````python
class Logic(IClientHandler):
    gameState: GameState

    def calculate_move(self) -> Move:
        possibleMoves = self.gameState.possible_moves
        return possibleMoves[0]

    def on_update(self, state: GameState):
        self.gameState = state

    def on_error(self, logMessage: str):
        ...
````

The above example is the simplest working Logic you can build. As you can see the Logic must inherit from
the `IClientHandler`, so that you can overwrite its methods and the api knows where to find your logic.

If you're done with your version of an working player, than you have to finish your file with this function, where you
call the Starter with your desired arguments. The following starts the client with the default arguments.

````python
if __name__ == "__main__":
    Starter(Logic())
````

If you want to run your logic from the console,
you can of course pass start arguments.
> Note that any arguments passed as startup parameters will override those in the code,
> including the ones you set yourself.

The following arguments are available:

- ``--help``                         Print the help message.
- ``--host <host>``                  The host to connect to. The default is *localhost*.
- ``--port <port>``                  The port of the host. The default is *13050*.
- ``--reservation <reservation>``    Reservation code for a prepared game.
- ``--room <room id>``               Room Id the client will try to connect.
- ``--keep_alive``                   If present the client will keep running,
  even if the connection to the server is terminated.
- ``--write_log``                    If present the client will write a log file to the current directory.