<a target="_blank" rel="noopener noreferrer" href="https://www.software-challenge.de"><p align="center"><img width="128" src="https://software-challenge.de/site/themes/freebird/img/logo.png" alt="Software-Challenge Logo"></p></a>

# Python-Client für die Software-Challenge Germany 2025

[![Read the Docs](https://img.shields.io/readthedocs/socha-python-client?label=Docs)](https://socha-python-client.readthedocs.io/en/)
[![PyPI](https://img.shields.io/pypi/v/socha?label=PyPi)](https://pypi.org/project/socha/)
[![PyPI - Python Version](https://img.shields.io/pypi/pyversions/socha?label=Python)](https://pypi.org/project/socha/)
[![Discord](https://img.shields.io/discord/233577109363097601?color=blue&label=Discord)](https://discord.gg/ARZamDptG5)
[![Documentation](https://img.shields.io/badge/Software--Challenge%20-Documentation-%234299e1)](https://docs.software-challenge.de/)

> Dieses Paket befindet sich derzeit in einem frühen Entwicklungsstadium!

Dieses Repository enthält das Python-Paket für die [Software-Challenge Germany](https://www.software-challenge.de), einem Programmierwettbewerb für Schülerinnen und Schüler. Dabei muss eine künstliche Intelligenz entwickelt werden, die in einem jährlich wechselnden Spiel gegen andere Gegner antritt.

> In diesem Jahr ist es das Spiel **[Hase und Igel](https://docs.software-challenge.de/spiele/25_hase-und-igel)**.

## Inhaltsverzeichnis

- [Installation](#installation)
  - [Global](#global)
  - [Virtuelle Umgebung](#virtuelle-umgebung)
- [Erste Schritte](#erste-schritte)
  - [Startargumente](#start-arguments)
- [Vorbereitung des Spielers auf den Wettbewerb](#vorbereitung-des-spielers-für-den-wettbewerb)
- [Lokale Entwicklung](#lokale-entwicklung)

## Installation

Es gibt zwei Methoden, um den `socha`-Client zu installieren. Die erste Methode ist die schnellste, um sofort loslegen zu können. Diese Methode eignet sich jedoch nicht, um den Player im Wettbewerbssystem zu betreiben, da es keine Internetverbindung gibt, über die Pakete heruntergeladen werden können. Daher wird die Installation in einer virtuellen Umgebung empfohlen, bei der die Pakete in einem Ordner installiert werden.

> Bitte sicherstellen, dass mindestens **Python 3.10** installiert ist. Dies kann mit `$ python -V` oder `$ python3 -V` überprüft werden.
>
> Falls Python nicht vorhanden ist, kann es mit folgenden Befehlen installiert werden:
>
> - Windows: `> winget install -e --id Python.Python.3.10`
> - Debian: `$ sudo apt install python3.10`
> - Arch: `$ sudo pacman -S python`

> In seltenen Fällen kann es zu einer [fehlerhaften oder fehlenden Installation von `setuptools`](https://github.com/maxblan/socha-python-client/issues/40) kommen. In diesem Fall kann `setuptools` mit `pip install -I setuptools` erneut installiert werden.

### Global

Die Installation ist mit pip recht einfach.

```shell
$ pip install socha
```

Wenn das Paket manuell installiert werden soll, muss die gewünschte Version heruntergeladen, das Paket entpackt und dann `setup.py` mit Python ausgeführt werden.

```shell
$ python setup.py install --user
```

Damit sollten die Abhängigkeiten erfüllt sein und das Paket ist sofort einsatzbereit.

### Virtuelle Umgebung

Um eine virtuelle Umgebung zu erstellen, sollte zunächst ein Verzeichnis erstellt und betreten werden, in dem der Player entwickelt werden soll.

```shell
$ mkdir my_player
$ cd my_player
```

Nun kann die virtuelle Umgebung (venv) erstellt werden.

```shell
$ python -m venv venv/
```

Nach Erstellung der _venv_ kann sie aktiviert werden.

Unter Linux:

```shell
$ source venv/bin/activate
```

Unter Windows:

```bash
> Set-ExecutionPolicy Unrestricted -Scope Process
> .venv\Scripts\activate
```

Die _venv_ sollte nun geöffnet sein und Pakete können installiert sowie Skripte ausgeführt werden. Um den Player zu entwickeln, muss das Paket socha mit `pip` installiert werden.

```shell
(venv) $ pip install socha
```

Damit sollten die Abhängigkeiten erfüllt sein und das Paket ist sofort einsatzbereit.

## Erste Schritte

Die Struktur der Klasse zur Entwicklung und Implementierung der Logik sollte wie folgt aussehen:

```python
class Logic(IClientHandler):
    GameState: GameState

    def calculate_move(self) -> Move:
        return Move(action=Advance(distance=1, cards=[]))

    def on_update(self, state: GameState):
        self.gameState = state
```

Das obige Beispiel zeigt die einfachste funktionierende Logik. Die Logik muss von `IClientHandler` erben, damit dessen Methoden überschrieben werden können und die API weiß, wo die Logik zu finden ist.

Wenn eine funktionierende Version des Players fertiggestellt ist, sollte die Datei mit dieser Funktion beendet werden, um den Starter mit den gewünschten Argumenten aufzurufen. Der folgende Code startet den Client mit den Standardargumenten.

```python
if __name__ == "__main__":
    Starter(Logik())
```

> Ein komplettes Beispiel ist in dieser [`logic.py`](https://github.com/maxblan/socha-python-client/blob/master/logic.py) zu finden.

### Startargumente

Falls die Logik von der Konsole aus ausgeführt werden soll, können Startargumente übergeben werden.

> Beachten, dass alle als Startparameter übergebenen Argumente die im Code gesetzten überschreiben, einschließlich derjenigen, die selbst gesetzt wurden.

| **Befehl**             | **Beschreibung**                                                                                 |
|------------------------|--------------------------------------------------------------------------------------------------|
| **--help**             | Druckt die Hilfemeldung.                                                                         |
| **-h, --host**         | Der Host, zu dem eine Verbindung hergestellt werden soll. Die Vorgabe ist 'localhost'.           |
| **-p, --port**         | Der Port des Hosts. Die Vorgabe ist 13050.                                                       |
| **-r, --reservation**  | Reservierungscode für ein vorbereitetes Spiel.                                                   |
| **-R, --room**         | Raumnummer, mit der der Client versucht, eine Verbindung herzustellen.                           |
| **-s, --survive**      | Falls vorhanden, läuft der Client weiter, auch wenn die Verbindung zum Server unterbrochen wird. |
| **-l, --log**          | Falls vorhanden, schreibt der Client eine Protokolldatei in das aktuelle Verzeichnis.            |
| **-v, --verbose**      | Ausführliche Option für die Protokollierung.                                                     |
| **--auto-reconnect**   | Verbindet sich automatisch wieder mit dem Server, wenn die Verbindung unterbrochen wird.         |
| **-b, --build**        | Baut dieses Skript zu einem Paket mit all seinen Abhängigkeiten.                                 |
| **-d, --directory**    | Das Verzeichnis, in dem das Paket erstellt werden soll.                                          |
| **-a, --architecture** | Die Architektur des Pakets.                                                                      |

## Vorbereitung des Spielers für den Wettbewerb

> Das Wettbewerbssystem läuft auf einem Linux-System mit einer `x86_64`-Architektur. Um den Client auf dem Wettbewerbssystem zu verwenden, muss das Socha-Paket für die Plattform `manylinux2014_x86_64` und die Python-Version `310` heruntergeladen werden.

Um sicherzustellen, dass der Player im Wettbewerbssystem verwendbar ist, müssen alle Abhängigkeiten heruntergeladen werden, da das System auf einem Docker-Container ohne Internetzugang und sudo-Berechtigung ausgeführt wird.

> Das Paket erleichtert die Vorbereitung! Eine Datei `requirements.txt`, die alle Abhängigkeiten auflistet, wird benötigt. Zum Starten folgenden Befehl im Terminal ausführen:
>
> `$ python <dein_haupt_skript>.py --build -directory <dein_ordner> -architecture <ziel_architektur>`
>
> Dadurch wird das Paket aktiviert und das Projekt erstellt.

Falls eine manuelle Vorgehensweise bevorzugt wird, folgen diese Schritte zum Herunterladen der Abhängigkeiten:

1. Terminal oder Konsole an dem Ort öffnen, an dem das Verzeichnis erstellt werden soll, das hochgeladen wird.
2. `mkdir my_player` eingeben, um ein neues Verzeichnis namens `my_player` zu erstellen. Der Verzeichnisname kann beliebig gewählt werden.
3. Mit `cd my_player` in das Verzeichnis wechseln.
4. Den Befehl `pip download socha --only-binary=:all: --platform manylinux2014_x86_64 --python-version 310 -d dependencies` im Verzeichnis ausführen, um die benötigten Abhängigkeiten in den Ordner `dependencies` herunterzuladen.
5. Alle Abhängigkeiten hinzufügen, die der Client verwendet.
6. Ein letztes Verzeichnis mit `mkdir .pip_cache` erstellen.

Nach dem Herunterladen der Abhängigkeiten muss ein Shell-Skript erstellt werden, das als Einstiegspunkt für den Player verwendet wird. Es **muss** den Namen `start.sh` tragen und sich auf der obersten Ebene des Verzeichnisses befinden, sonst kann es nicht gefunden werden.

Um das Shell-Skript zu erstellen, sind folgende Schritte notwendig:

1. Das Shell-Skript in einer UNIX-Umgebung erstellen.

 Unter Windows kann WSL oder Notepad++ verwendet werden. In Notepad++ zu _Bearbeiten->Format Zeilenende->Unix(LF)_ wechseln, um sicherzustellen, dass die Zeilenenden nur `LS` ohne `CR` sind.

2. Das Shell-Skript sollte die folgende Struktur haben:

```shell
#!/bin/sh

# Sofortiges Beenden, wenn ein Befehl fehlschlägt
set -e

# Setzt die Umgebungsvariable, die den Ort angibt, an dem pip seine Cache-Dateien speichert
export XDG_CACHE_HOME=./my_player/.pip_cache

# Setzt die Umgebungsvariable, die das Verzeichnis zur Liste der Pfade hinzufügt, die Python nach Modulen und Paketen durchsucht, wenn diese importiert werden.
export PYTHONPATH=./my_player/packages:$PYTHONPATH

# Installieren Sie das Paket socha
pip install --no-index --find-links=./my_player/dependencies/ ./my_player/dependencies/socha-1.0.1-py3-none-any.whl ./my_player/dependencies/xsdata-22.7-py3-none-any.whl --target=./my_player/packages/ --cache-dir=./my_player/.pip_cache

# Das Skript logic.py mit Startargumenten ausführen
python3 ./my_player/logic.py "$@"
```

3. Alle Abhängigkeiten, die der Client verwendet, zu diesem Skript hinzufügen.

Nach Erstellung des Shell-Skripts sollte die Verzeichnisstruktur wie folgt aussehen:

```
my_player/
├── .pip_cache/
├─── dependencies/
├─── logic.py
└── start.sh
```

Das Verzeichnis `my_player` (oder wie auch immer es benannt wurde) muss nur noch als ZIP-Archiv verpackt werden, und der Player ist bereit zum Hochladen. Herzlichen Glückwunsch! 🥳🎉

## Lokale Entwicklung

> 🏗️ Dieser Teil ist derzeit noch unfertig und kann sich noch ändern.

Dieses Paket wurde größtenteils in Rust geschrieben, was einen deutlichen Leistungsschub im Vergleich zu einem nativen Python-Programm bringt. Allerdings führt dies zu einem erheblichen Aufwand, da sogenannte Bindings erstellt werden müssen, die es Python ermöglichen, auf die Funktionen in Rust zuzugreifen. Hierfür wird [PyO3](https://github.com/PyO3/pyo3) mit Hilfe von [Maturin](https://github.com/PyO3/maturin) verwendet.

Für eine lokale Entwicklung müssen folgende Dinge installiert werden:

- [Rust Compiler mit Cargo](https://www.rust-lang.org/tools/install),
- [Python 3.10 oder höher](https://www.python.org/downloads/),
- und [Maturin](https://github.com/PyO3/maturin) in einer virtuellen Umgebung in diesem Repository.

Nach erfolgreicher Installation muss der Befehl `maturin develop` in einer virtuellen Umgebung ausgeführt werden. Dann kann eine in Python geschriebene Logik verwendet und Änderungen im Rust-Code vorgenommen werden. Nach jeder Änderung muss `maturin develop` erneut ausgeführt werden, damit die Änderungen für den Python-Code sichtbar werden.