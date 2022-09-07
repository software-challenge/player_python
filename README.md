<a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a>

# Python Client for the Software-Challenge Germany 2023

[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/FalconsSky/Software-Challenge-Python-Client/static%20and%20unit%20tests?label=Test)](https://github.com/FalconsSky/Software-Challenge-Python-Client)
[![Read the Docs](https://img.shields.io/readthedocs/software-challenge-python-client?label=Docs)](https://software-challenge-python-client.readthedocs.io/en/latest/)
[![PyPI](https://img.shields.io/pypi/v/socha?label=PyPi)](https://pypi.org/project/socha/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/socha?label=Python)](https://pypi.org/project/socha/)
[![Discord](https://img.shields.io/discord/233577109363097601?color=blue&label=Discord)](https://discord.gg/ARZamDptG5)
[![Documentation](https://img.shields.io/badge/Software--Challenge%20-Documentation-%234299e1)](https://docs.software-challenge.de/)
> Please note that this is a very early version, which may still contain some bugs. However, the client is able to play
> a game from start to end.

This repository contains the Python package for the
[Software-Challenge Germany](https://www.software-challenge.de), a programming competition for students. The students
have to develop an artificial intelligence that plays and competes against other opponents in an annually changing game.

> This year it is the game
> **[Hey, danke für den Fisch!](https://docs.software-challenge.de/spiele/penguins)**.

## Installation

Two methods are presented here to install the `socha` client.
The first one is the fastest to get started right away.
However,
this method will not make it possible to run your player in the competition system,
since there is no Internet connection that allows you to download packages.
Therefore,
the possibility of a virtual environment is presented,
which installs the packages inside the folder.

> Pleas make sure that you have at least **Python 3.6** installed.
> Check with `$ python -V` or `$ python3 -V`.
> - Windows: `> winget install -e --id Python.Python.3.6`
> - Debian: `$ sudo apt install python3.6`
> - Arch: `$ sudo pacman -S python`

### Globally

The installation is quite simple with pip.

```shell
$ pip install socha
```

If you want to install the package manually, then you have to download the release of your choice, unpack the package
and then run `setup.py` with Python.

```shell
$ python setup.py install --user
```

This should satisfy the dependencies and you can start right away.

### Virtual Environment

To create a virtual environment,
you should first create a directory in which you want to develop your player
and than enter that directory.

```shell
$ mkdir my_player
$ cd my_player
```

Now you can create the virtual environment (venv).

```shell
$ python -m venv venv/
```

This takes a moment. After the *venv* is created, you can open it.

On Linux:

```shell
$ source venv/bin/activate
```

On Windows:

```bash
> Set-ExecutionPolicy Unrestricted -Scope Process
> .\venv\Scripts\activate
```

It should open the *venv* and you can now install packages and run your scripts here.
To develop your player you just have to install the package socha with `pip`.

```shell
(venv) $ pip install socha
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
````

The above example is the simplest working Logic you can build. As you can see the Logic must inherit from
the `IClientHandler`, so that you can overwrite its methods and the api knows where to find your logic.

If you're done with your version of an working player, than you have to finish your file with this function, where you
call the Starter with your desired arguments. The following starts the client with the default arguments.

````python
if __name__ == "__main__":
    Starter(Logic())
````

### Start arguments

If you want to run your logic from the console,
you can of course pass start arguments.
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

## Make your player ready to hand in

To make your player usable for the competition system,
you need to create a virtual environment,
as described [above](#virtual-environment).

Once you have done this,
you still need to create a shell script
that uses the contest system as the entry point for your player.
It **must** be named `start.sh` because otherwise it cannot be found.
There you must enter the following and place it in the root of your directory.

```shell
#!/bin/sh
chmod +x logic.py
. venv/bin/activate
python ./logic.py "$@"
```

When you have done this,
you should have a directory structure that looks something like this:

````
my_player/
|- venv/
|- logic.py
|- start.sh
````

The `my_player` directory,
or whatever you named yours,
then just needs to be packaged as a ZIP archive
and your player is ready to be uploaded. 🥳🎉
