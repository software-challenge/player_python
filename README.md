<a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a>

# Python Client for the Software-Challenge Germany 2024

[![Discord](https://img.shields.io/discord/233577109363097601?color=blue&label=Discord)](https://discord.gg/ARZamDptG5)

> **Be warned!** This package is currently experimental and has still a few known bugs. But please try it out and let us know if anything comes up.

This repository contains the Python package for the
[Software-Challenge Germany](https://www.software-challenge.de), a programming competition for students. The students
have to develop an artificial intelligence that plays and competes against other opponents in an annually changing game.

> This year it is the game
> **[Mississippi Queen](https://docs.software-challenge.de/spiele/mississippi-queen)**.

## Table of Contents

- [Installation](#installation)
  - [Globally](#globally)
  - [Virtual Environment](#virtual-environment)
- [Getting Started](#getting-started)
  - [Start Arguments](#start-arguments)
- [Preparing Your Player for the Competition](#preparing-your-player-for-the-competition)
- [Local Development](#local-development)

## Installation

Two methods are presented here to install the `socha` client.
The first one is the fastest to get started right away.
However,
this method will not make it possible to run your player in the competition system,
since there is no Internet connection that allows you to download packages.
Therefore,
the possibility of a virtual environment is presented,
which installs the packages inside the folder.

> Pleas make sure that you have at least **Python 3.10** installed.
> Check with `$ python -V` or `$ python3 -V`.
>
> If not present you can install python with the following commands:
>
> - Windows: `> winget install -e --id Python.Python.3.10`
> - Debian: `$ sudo apt install python3.10`
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

This takes a moment. After the _venv_ is created, you can open it.

On Linux:

```shell
$ source venv/bin/activate
```

On Windows:

```bash
> Set-ExecutionPolicy Unrestricted -Scope Process
> .\venv\Scripts\activate
```

It should open the _venv_ and you can now install packages and run your scripts here.
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

```python
from socha import *
```

If you now want to develop and implement your logic, then the structure of the class should look like this.

```python
class Logic(IClientHandler):
    gameState: GameState

    def calculate_move(self) -> Move:
        possibleMoves = self.gameState.possible_moves
        return possibleMoves[0]

    def on_update(self, state: GameState):
        self.gameState = state
```

The above example is the simplest working Logic you can build. As you can see the Logic must inherit from
the `IClientHandler`, so that you can overwrite its methods and the api knows where to find your logic.

If you're done with your version of an working player, than you have to finish your file with this function, where you
call the Starter with your desired arguments. The following starts the client with the default arguments.

```python
if __name__ == "__main__":
    Starter(Logic())
```

### Start arguments

If you want to run your logic from the console,
you can of course pass start arguments.

> Note that any arguments passed as startup parameters will override those in the code,
> including the ones you set yourself.

| **Command**           | **Description**                                                                               |
| --------------------- | --------------------------------------------------------------------------------------------- |
| **--help**            | Prints the help message.                                                                      |
| **-h, --host**        | The host to connect to. The default is 'localhost'.                                           |
| **-p, --port**        | The port of the host. The default is 13050.                                                   |
| **-r, --reservation** | Reservation code for a prepared game.                                                         |
| **-R, --room**        | Room Id the client will try to connect.                                                       |
| **-s, --survive**     | If present, the client will keep running, even if the connection to the server is terminated. |
| **-l, --log**         | If present, the client will write a log file to the current directory.                        |
| **-v, --verbose**     | Verbose option for logging.                                                                   |
| **--auto-reconnect**  | Automatically reconnect to the server if the connection is lost.                              |
| **-b, --build**       | Builds this script into a package with all its dependencies.                                  |

## Preparing Your Player for the Competition

To ensure that your player is usable for the competition system,
you need to download all the dependencies that your client uses
because the system will run on a docker container without access to the internet and sudo permission.

> The package has made things easier for you! You can use it to handle almost everything by itself.
> All you need is a `requirements.txt` file that lists all your dependencies.
> To start, simply run the following command in your terminal:
>
> `$ python <your_main_script>.py --build <your_directory_name>`
>
> This will trigger the package to do its magic and build your project.

If you want to do it manually, follow the steps below to download the dependencies:

1. Open your terminal or console wherever you want to create your directory that you will upload.
2. Type `mkdir my_player` to create a new directory named `my_player`. You can name yours whatever you want.
3. Enter the directory using `cd my_player`.
4. Run the command: `pip download socha xsdata==22.7 -d dependencies` in the directory.
   This command downloads the dependencies you need into the folder `dependencies`.
5. Ensure to add all your dependencies that your client uses.
6. After the download, create a last directory using `mkdir .pip_cache`.

Once you have downloaded all the dependencies,
you need to create a shell script that uses the contest system as the entry point for your player.
It **must** be named `start.sh` and must be on the top level of your directory; otherwise, it cannot be found.
Follow the steps below to create your shell script:

1. Ensure that you create your shell script in a UNIX-Environment, or if you use Windows,
   you can do this with WSL or Notepad++. If you use Notepad++,
   you need to go to _Bearbeiten->Format Zeilenende->Unix(LF)_.
   This step ensures that your line endings are `LS` only without `CR`, which may cause problems on the contest system.

2. Ensure that your shell script has the following structure:

```shell
#!/bin/sh

# Exit immediately if any command fails
set -e

# Sets the environment variable, which specifies the location for pip to store its cache files
export XDG_CACHE_HOME=./my_player/.pip_cache

# Sets the environment variable, which adds the directory to the list of paths that Python searches for modules and packages when they are imported.
export PYTHONPATH=./my_player/packages:$PYTHONPATH

# Install the socha package
pip install --no-index --find-links=./my_player/dependencies/ ./my_player/dependencies/socha-1.0.1-py3-none-any.whl ./my_player/dependencies/xsdata-22.7-py3-none-any.whl --target=./my_player/packages/ --cache-dir=./my_player/.pip_cache

# Run the logic.py script with start arguments
python3 ./my_player/logic.py "$@"
```

3. Ensure that you add all your dependencies that your client is using to this script.

Once you have created your shell script, you should have a directory structure that looks like this:

```
my_player/
├── .pip_cache/
├── dependencies/
├── logic.py
└── start.sh
```

The `my_player` directory or whatever you named yours just needs to be packaged as a ZIP archive,
and your player is ready to be uploaded. Congratulations! 🥳🎉


## Local Development
> 🏗️ This part is currently still unfinished and subject to change.

This package was mostly written in Rust, which gives a significant performance boost compared to a natural Python program.
However, this leads to considerable effort, as so-called bindings have to be created. These allow Python to access the functions in Rust. To realize this, [PyO3](https://github.com/PyO3/pyo3) is used here with the help of [Maturin](https://github.com/PyO3/maturin). 

If local development is desired, the following things must be installed beforehand:
- [Rust Compiler with Cargo](https://www.rust-lang.org/tools/install),
- [Python 3.10 or later](https://www.python.org/downloads/),
- and [Maturin](https://github.com/PyO3/maturin) in a virtual environment in this repository.

If everything has been installed successfully, then the command `maturin develop` must be executed in a virtual environment.
Now you can use a logic written in Python and make changes in the Rust code. Each time a change is made, `maturin develop` must be executed again to make the change visible to the Python code.