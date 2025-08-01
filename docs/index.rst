.. raw:: html

   <!-- trunk-ignore-all(markdownlint/MD041) -->

Python Client for the Software-Challenge Germany 2026
=====================================================

|Read the Docs| |PyPI| |PyPI - Python Version| |Discord| |Documentation|

   This package is currently in early stage of development!

This repository contains the Python package for the `Software-Challenge
Germany <https://www.software-challenge.de>`__, a programming
competition for students. The students have to develop an artificial
intelligence that plays and competes against other opponents in an
annually changing game.

   This year it is the game `Piranhas <>`__.

Table of Contents
-----------------

-  `Installation <#installation>`__

   -  `Globally <#globally>`__
   -  `Virtual Environment <#virtual-environment>`__

-  `Getting Started <#getting-started>`__

   -  `Start Arguments <#start-arguments>`__

-  `Preparing Your Player for the
   Competition <#preparing-your-player-for-the-competition>`__
-  `Local Development <#local-development>`__

Installation
------------

Two methods are presented here to install the ``socha`` client. The
first one is the fastest to get started right away. However, this method
will not make it possible to run your player in the competition system,
since there is no Internet connection that allows you to download
packages. Therefore, the possibility of a virtual environment is
presented, which installs the packages inside the folder.

   Pleas make sure that you have at least **Python 3.10** installed.
   Check with ``$ python -V`` or ``$ python3 -V``.

   If not present you can install python with the following commands:

   -  Windows: ``> winget install -e --id Python.Python.3.10``
   -  Debian: ``$ sudo apt install python3.10``
   -  Arch: ``$ sudo pacman -S python``

..

   In some rare cases there maybe a `broken or missing installation of
   ``setuptools`` <https://github.com/maxblan/socha-python-client/issues/40>`__.
   If this is the case on your machine you can install it again with
   ``pip install -I setuptools``.

Globally
~~~~~~~~

The installation is quite simple with pip.

.. code:: shell

   $ pip install socha

If you want to install the package manually, then you have to download
the release of your choice, unpack the package and then run ``setup.py``
with Python.

.. code:: shell

   $ python setup.py install --user

This should satisfy the dependencies and you can start right away.

Virtual Environment
~~~~~~~~~~~~~~~~~~~

To create a virtual environment, you should first create a directory in
which you want to develop your player and than enter that directory.

.. code:: shell

   $ mkdir my_player
   $ cd my_player

Now you can create the virtual environment (venv).

.. code:: shell

   $ python -m venv venv/

This takes a moment. After the *venv* is created, you can open it.

On Linux:

.. code:: shell

   $ source venv/bin/activate

On Windows:

.. code:: bash

   > Set-ExecutionPolicy Unrestricted -Scope Process
   > .\venv\Scripts\activate

It should open the *venv* and you can now install packages and run your
scripts here. To develop your player you just have to install the
package socha with ``pip``.

.. code:: shell

   (venv) $ pip install socha

This should satisfy the dependencies and you can start right away.

Getting Started
---------------

If you now want to develop and implement your logic, then the structure
of the class should look like this.

.. code:: python

   class Logic(IClientHandler):
       gameState: GameState

       def calculate_move(self) -> Move:
           return Move(action=Advance(distance=1, cards=[]))

       def on_update(self, state: GameState):
           self.gameState = state

The above example is the simplest working Logic you can build. As you
can see the Logic must inherit from the ``IClientHandler``, so that you
can overwrite its methods and the api knows where to find your logic.

If you're done with your version of an working player, than you have to
finish your file with this function, where you call the Starter with
your desired arguments. The following starts the client with the default
arguments.

.. code:: python

   if __name__ == "__main__":
       Starter(Logic())

..

   If you want a complete file as an example, you can take a look at
   this
   ```logic.py`` <https://github.com/maxblan/socha-python-client/blob/master/logic.py>`__.

Start arguments
~~~~~~~~~~~~~~~

If you want to run your logic from the console, you can of course pass
start arguments.

   Note that any arguments passed as startup parameters will override
   those in the code, including the ones you set yourself.

+------------------------+--------------------------------------------+
| **Command**            | **Description**                            |
+========================+============================================+
| **--help**             | Prints the help message.                   |
+------------------------+--------------------------------------------+
| **-h, --host**         | The host to connect to. The default is     |
|                        | 'localhost'.                               |
+------------------------+--------------------------------------------+
| **-p, --port**         | The port of the host. The default is       |
|                        | 13050.                                     |
+------------------------+--------------------------------------------+
| **-r, --reservation**  | Reservation code for a prepared game.      |
+------------------------+--------------------------------------------+
| **-R, --room**         | Room Id the client will try to connect.    |
+------------------------+--------------------------------------------+
| **-s, --survive**      | If present, the client will keep running,  |
|                        | even if the connection to the server is    |
|                        | terminated.                                |
+------------------------+--------------------------------------------+
| **-l, --log**          | If present, the client will write a log    |
|                        | file to the current directory.             |
+------------------------+--------------------------------------------+
| **-v, --verbose**      | Verbose option for logging.                |
+------------------------+--------------------------------------------+
| **--auto-reconnect**   | Automatically reconnect to the server if   |
|                        | the connection is lost.                    |
+------------------------+--------------------------------------------+
| **-b, --build**        | Builds this script into a package with all |
|                        | its dependencies.                          |
+------------------------+--------------------------------------------+
| **-d, --directory**    | The directory where the package should be  |
|                        | built.                                     |
+------------------------+--------------------------------------------+
| **-a, --architecture** | The architecture of the package.           |
+------------------------+--------------------------------------------+
| **--python-version**   | Specifies the build python version         |
|                        | (e.g.: '3.10' - this is standard).         |
+------------------------+--------------------------------------------+

Preparing Your Player for the Competition
-----------------------------------------

   Please note that the competition system runs on a Linux system with
   an ``x86_64`` architecture. To use your client on the competition
   system, you will need to download the socha package built for the
   ``manylinux2014_x86_64`` platform and the Python version ``310``.

To ensure that your player is usable for the competition system, you
need to download all the dependencies that your client uses because the
system will run on a docker container without access to the internet and
sudo permission.

   The package has made things easier for you! You can use it to handle
   almost everything by itself. All you need is a ``requirements.txt``
   file that lists all your dependencies. To start, simply run the
   following command in your terminal:

   ``$ python <your_main_script>.py --build -directory <your_directory_name> -architecture <target architecture>``

   This will trigger the package to do its magic and build your project.

If you want to do it manually, follow the steps below to download the
dependencies:

1. Open your terminal or console wherever you want to create your
   directory that you will upload.
2. Type ``mkdir my_player`` to create a new directory named
   ``my_player``. You can name yours whatever you want.
3. Enter the directory using ``cd my_player``.
4. Run the command:
   ``pip download socha --only-binary=:all: --platform manylinux2014_x86_64 --python-version 310 -d dependencies``
   in the directory. This command downloads the dependencies you need
   into the folder ``dependencies``.
5. Ensure to add all your dependencies that your client uses.
6. After the download, create a last directory using
   ``mkdir .pip_cache``.

Once you have downloaded all the dependencies, you need to create a
shell script that uses the contest system as the entry point for your
player. It **must** be named ``start.sh`` and must be on the top level
of your directory; otherwise, it cannot be found. Follow the steps below
to create your shell script:

1. Ensure that you create your shell script in a UNIX-Environment, or if
   you use Windows, you can do this with WSL or Notepad++. If you use
   Notepad++, you need to go to *Bearbeiten->Format
   Zeilenende->Unix(LF)*. This step ensures that your line endings are
   ``LS`` only without ``CR``, which may cause problems on the contest
   system.

2. Ensure that your shell script has the following structure:

.. code:: shell

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

3. Ensure that you add all your dependencies that your client is using
   to this script.

Once you have created your shell script, you should have a directory
structure that looks like this:

::

   my_player/
   ├── .pip_cache/
   ├── dependencies/
   ├── logic.py
   └── start.sh

The ``my_player`` directory or whatever you named yours just needs to be
packaged as a ZIP archive, and your player is ready to be uploaded.
Congratulations! 🥳🎉

Local Development
-----------------

   🏗️ This part is currently still unfinished and subject to change.

This package was mostly written in Rust, which gives a significant
performance boost compared to a natural Python program. However, this
leads to considerable effort, as so-called bindings have to be created.
These allow Python to access the functions in Rust. To realize this,
`PyO3 <https://github.com/PyO3/pyo3>`__ is used here with the help of
`Maturin <https://github.com/PyO3/maturin>`__.

If local development is desired, the following things must be installed
beforehand:

-  `Rust Compiler with
   Cargo <https://www.rust-lang.org/tools/install>`__,
-  `Python 3.10 or later <https://www.python.org/downloads/>`__,
-  and `Maturin <https://github.com/PyO3/maturin>`__ in a virtual
   environment in this repository.

If everything has been installed successfully, then the command
``maturin develop`` must be executed in a virtual environment. Now you
can use a logic written in Python and make changes in the Rust code.
Each time a change is made, ``maturin develop`` must be executed again
to make the change visible to the Python code.

.. |Read the Docs| image:: https://img.shields.io/readthedocs/socha-python-client?label=Docs
   :target: https://socha-python-client.readthedocs.io/en/
.. |PyPI| image:: https://img.shields.io/pypi/v/socha?label=PyPi
   :target: https://pypi.org/project/socha/
.. |PyPI - Python Version| image:: https://img.shields.io/pypi/pyversions/socha?label=Python
   :target: https://pypi.org/project/socha/
.. |Discord| image:: https://img.shields.io/discord/233577109363097601?color=blue&label=Discord
   :target: https://discord.gg/ARZamDptG5
.. |Documentation| image:: https://img.shields.io/badge/Software--Challenge%20-Documentation-%234299e1
   :target: https://docs.software-challenge.de/


Indices and tables
==================

.. toctree::
   :maxdepth: 5
   :caption: Contents:

   socha


* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
