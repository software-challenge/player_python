<a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a>

# Python Client for the Software-Challenge Germany 2023

[![Read the Docs](https://img.shields.io/readthedocs/software-challenge-python-client?label=Docs)](https://software-challenge-python-client.readthedocs.io/en/latest/)
[![PyPI](https://img.shields.io/pypi/v/socha?label=PyPi)](https://pypi.org/project/socha/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/socha?label=Python)](https://pypi.org/project/socha/)
[![Discord](https://img.shields.io/discord/233577109363097601?color=blue&label=Discord)](https://discord.gg/ARZamDptG5)
[![Documentation](https://img.shields.io/badge/Software--Challenge%20-Documentation-%234299e1)](https://docs.software-challenge.de/)

> Please note that this is a version with Python bindings written in Rust.
> This package is highly **experimental** and is currently released mainly for testing.
> In addition, a few methods and classes **differ** from the pure python version, which is due to the nature of Rust.
> Furthermore, it should be noted that at least Python **3.7** is required for this package.

This repository contains the Python package for the
[Software-Challenge Germany](https://www.software-challenge.de), a programming competition for students. The students
have to develop an artificial intelligence that plays and competes against other opponents in an annually changing game.

# Game
This year, the game is Hey, danke für den Fisch!.

Hey, danke für den Fisch is a board game in which two players compete against each other on a hexagonal field. 
At the beginning of the game, fish are randomly distributed on the field. In the first moves, only penguins are placed. 
Penguins can be moved in straight paths across the axes of the field, 
but they may not slide across fields without ice floes or across other penguins. 
If a penguin slides off a field, it collects the field's fish and destroys the ice floe. 
The game ends when both players can no longer move. 
The player who has collected more fish during the game wins.

# Installation
Two methods are presented here to install the socha client. The first one is the fastest to get started right away, but will not make it possible to run your player in the competition system. The second method creates a virtual environment that installs the packages inside the folder.

## Prerequisites
Make sure you have at least Python 3.7 installed. Check with python -V or python3 -V. If Python is not installed, use the following commands to install it:

- Windows: ``> winget install -e --id Python.Python.3.7``
- Debian: ``$ sudo apt install python3.7``
- Arch: ``$ sudo pacman -S python``

To download the wheel for this package follow these steps:

1. Go to the [``FalconsSky/socha-python-client``](https://github.com/FalconsSky/socha-python-client) repository
2. Navigate to the [``rust/master``](https://github.com/FalconsSky/socha-python-client/tree/rust/master) branch where the package is located.
3. Open the [`dist/`](https://github.com/FalconsSky/socha-python-client/tree/rust/master/dist) directory.
4. Choose the correct package version for your Python version (3.7 to 3.11) and operating system (Linux, Windows, or Mac OS). The package name will help you identify the correct version.
5. Download the wheel file.

## Method 1: Globally
Use pip to install the package globally. 
The installation process should be straightforward once you have downloaded the wheel.
```shell
pip install path/to/package/wheel
```
## Method 2: Virtual Environment
To create a virtual environment and install the socha client in it, follow these steps:

1. Create a directory where you want to develop your player: ``mkdir my_player``
2. Navigate to the directory: ``cd my_player``
3. Create the virtual environment: ``python -m venv venv/``
4. Open the virtual environment:
   - On Linux:
   ```shell
    $ source venv/bin/activate
   ```
   - On Windows:
    ```shell
    > Set-ExecutionPolicy Unrestricted -Scope Process
    > .\venv\Scripts\activate
   ```
5. Install the ``socha`` client:
   ```shell
    (venv) $ pip install path/to/package/wheel
   ```
# Getting Started

In order to use the Software-Challenge Python Client, 
you will need to import the following dependencies:

````python
from socha import *
from socha.api.networking.player_client import IClientHandler
from socha.starter import Starter
````

To develop and implement your logic, create a class that inherits from ``IClientHandler`` and overrides its methods. 
Here is an example of the simplest working logic you can build:

````python
class Logic(IClientHandler):
    gameState: GameState

    def calculate_move(self) -> Move:
        possibleMoves = self.gameState.possible_moves
        return possibleMoves[0]

    def on_update(self, state: GameState):
        self.gameState = state
````

Once you have finished developing your player, you can run it using the Starter function. 
The following example will start the client with the default arguments:

````python
if __name__ == "__main__":
    Starter(Logic())
````

# Start arguments

You can pass start arguments through the console to customize the execution of your player.
> Note that any arguments passed as startup parameters will override those in the code,
> including the ones you set yourself.

| argument                                         | description                                                                                  |
|--------------------------------------------------|----------------------------------------------------------------------------------------------|
| `--help `                                        | Prints a help message.                                                                       |
| `-h HOST ` ,  `--host HOST `                     | The host to connect to. The default is 'localhost'.                                          |
| `-p PORT `,  `--port PORT `                      | The port of the host. The default is 13050.                                                  |
| `-r RESERVATION `,  `--reservation RESERVATION ` | Reservation code for a prepared game.                                                        |
| `-R ROOM `,  `--room ROOM `                      | Room Id the client will try to connect.                                                      |
| `-s `,  `--survive `                             | If present the client will keep running, even if the connection to the server is terminated. |
| `-l `,  `--log `                                 | If present the client will write a log file to the current directory.                        |
| `-v `,  `--verbose `                             | Verbose option for logging.                                                                  |

# Prepare Your Player for Submission

To make your player ready for submission to the competition system, you will need to do the following:

1. Create a virtual environment, as described in the [previous section](#method-2:-virtual-environment).
2. Create a shell script named start.sh in the root directory of your player. This script will be the entry point for your player and must be named start.sh for it to be found. 
The contents of the script should be as follows:

```shell
#!/bin/sh
. venv/bin/activate
python ./logic.py "$@"
```

Your player's directory structure should look like this:

````
my_player/
├── venv/
├── logic.py
└── start.sh
````

Once you have completed these steps, 
you can package your player as a ZIP archive and submit it. 🥳🎉
