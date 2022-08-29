"""
This is the main entry point for the SoCha application.
"""
import datetime
import logging
import sys

from socha.api.networking.player_client import _PlayerClient, IClientHandler


class Starter:
    """
    When this is called, the client will try to connect to the server and join a game.
    When successful, the client will start the loop and call the on_update and calculate_move methods,
    if the server sends updates.
    """

    def __init__(self, logic: IClientHandler, host: str = "localhost", port: int = 13050, reservation: str = None,
                 room_id: str = None, keep_alive: bool = False, write_log: bool = False):
        """
        All these arguments can be overwritten, when parsed via start arguments,
        or you initialize this class with the desired values.

        :param logic: Your logic the client will call, if moves are requested.
        :param host: The host that the client should connect to.
        :param port: The port of the host.
        :param reservation: Reservation code for a prepared game.
        :param room_id: Room Id the client will try to connect.
        :param keep_alive: If True the client will keep running, even if the connection to the server is terminated.
        :param write_log: If True the client will write a log file to the current directory.
        """
        args = self._handle_start_args()

        self.host: str = host
        self.port: int = port
        self.reservation: str = reservation
        self.room_id: str = room_id
        self.keep_alive: bool = keep_alive
        self.write_log: bool = write_log

        for key, value in args.items():
            if value:
                setattr(self, key, value)

        if write_log:
            now = datetime.datetime.now().strftime("%Y%m%d%H%M%S")
            logging.basicConfig(filename=f"log{now}", level=logging.INFO)
            logging.getLogger().addHandler(logging.StreamHandler())
        else:
            logging.basicConfig(level=logging.INFO)
        logging.info("Starting...")

        self.client = _PlayerClient(host=self.host, port=self.port, handler=logic, keep_alive=self.keep_alive)

        if reservation:
            self.client.join_game_with_reservation(reservation)
        elif room_id:
            self.client.join_game_room(room_id)
        else:
            self.client.join_game()

        self.client.start()

    _arguments = {
        "help": None,
        "host": None,
        "port": None,
        "reservation": None,
        "room": None,
        "keep_alive": None,
        "write_log": None
    }

    def _handle_start_args(self):
        if len(sys.argv) > 1:
            if sys.argv[1] == "--help":
                self._show_help()
                sys.exit(0)
            for i, argument in enumerate(sys.argv):
                arg = argument.replace("--", "")
                if arg in self._arguments:
                    if arg == "port":
                        self._arguments[arg] = int(sys.argv[i + 1])
                    elif arg == "keep-alive":
                        self._arguments[arg] = True
                    elif arg == "write-log":
                        self._arguments[arg] = True
                    else:
                        self._arguments[arg] = sys.argv[i + 1]
        return self._arguments

    @staticmethod
    def _show_help():
        print("Usage:\n"
              "python logic.py [--help] [--host <host>] [--port <port>] [--reservation <reservation>] "
              "[--room <room id>] [--keep-alive] [--write-log]\n"
              "---------------------------------------------------------\n"
              "Options:\n"
              "  --help                         Print this help message.\n"
              "  --host <host>                  The host to connect to. The default is 'localhost'\n"
              "  --port <port>                  The port of the host. The default is 13050.\n"
              "  --reservation <reservation>    Reservation code for a prepared game.\n"
              "  --room <room id>               Room Id the client will try to connect.\n"
              "  --keep_alive                   If present the client will keep running, even if the connection to"
              " the server is terminated.\n"
              "  --write_log                    If present the client will write a log file to the current "
              "directory.\n")
