"""
This is the main entry point for the SoCha application.
"""

import argparse
import datetime
import json
import logging
import urllib.request

import pkg_resources
from socha.api.networking.game_client import GameClient, IClientHandler
from socha.utils.package_builder import SochaPackageBuilder


class Starter:
    """
    When this is called, the client will try to connect to the server and join a game.
    When successful, the client will start the loop and call the on_update and calculate_move methods,
    if the server sends updates.
    """

    def __init__(
        self,
        logic: IClientHandler,
        host: str = "localhost",
        port: int = 13050,
        reservation: str = None,
        room_id: str = None,
        password: str = None,
        survive: bool = False,
        auto_reconnect: bool = False,
        headless: bool = False,
        log: bool = False,
        verbose: bool = False,
        build: bool = False,
        directory: str = None,
        architecture: str = None,
        log_level: int = logging.INFO,
    ):
        """
        All these arguments can be overwritten, when parsed via start arguments,
        or you initialize this class with the desired values.

        Args:
            logic: Your logic the client will call, if moves are requested.
            host: The host that the client should connect to.
            port: The port of the host.
            reservation: Reservation code for a prepared game.
            room_id: Room ID the client will try to connect.
            password: Password for the server for authentication as admin.
            survive: If True the client keep running, even if the connection to the server is terminated.
            auto_reconnect: If True the client will try to reconnect to the server, if the connection is lost.
            headless: If True the client will not use the penguin plugin.
            log: If True the client write a log file to the current directory.
            verbose: Verbose option for logging.
            build: If set, the client will build a zip package with the given name.
        """
        VERBOSE = 15
        logging.addLevelName(VERBOSE, "VERBOSE")

        args = self._handle_start_args()

        self.write_log: bool = args.log or log
        self.verbose = args.verbose or verbose
        self.log_level = args.log_level or log_level
        self._setup_debugger(self.verbose, self.log_level)

        self.check_socha_version()

        self.directory: str = args.directory or directory
        self.architecture: str = args.architecture or architecture
        self.build: str = args.build or build
        if self.build:
            builder = SochaPackageBuilder(self.directory, self.architecture)
            builder.build_package()
            exit(0)

        self.host: str = args.host or host
        self.port: int = args.port or port
        self.reservation: str = args.reservation or reservation
        self.room_id: str = args.room or room_id
        if self.room_id and self.reservation:
            logging.warning(
                "The room ID is not taken into account because a reservation is available."
            )
        self.password: str = args.password or password
        if self.password and (self.reservation or self.room_id):
            logging.warning(
                "The password is not taken into account because a reservation or Room ID is available."
            )
        self.survive: bool = args.survive or survive
        self.auto_reconnect: bool = args.auto_reconnect or auto_reconnect
        self.headless: bool = args.headless or headless

        self.client = GameClient(
            host=self.host,
            port=self.port,
            handler=logic,
            reservation=self.reservation,
            room_id=room_id,
            password=self.password,
            auto_reconnect=self.auto_reconnect,
            survive=self.survive,
            headless=self.headless,
        )

        self.client.join()

        self.client.start()

    def _setup_debugger(self, verbose: bool, log_level: int):
        if verbose:
            level: int = logging.DEBUG
        else:
            level: int = log_level

        if self.write_log:
            now = datetime.datetime.now().strftime("%Y%m%d%H%M%S")
            logging.basicConfig(
                filename=f"log{now}",
                level=level,
                format="%(asctime)s: %(levelname)s - %(message)s",
            )
            logging.getLogger().addHandler(logging.StreamHandler())
        else:
            logging.basicConfig(
                level=level, format="%(asctime)s: %(levelname)s - %(message)s"
            )
        logging.info("Starting...")
        logging.info(
            "We would greatly appreciate it if you could share any issues "
            "or feature requests you may have regarding socha by either creating "
            "an issue on our GitHub repository or contributing to the project."
            "\n(https://github.com/maxblan/socha-python-client)"
        )

    @staticmethod
    def check_socha_version():
        package_name = "socha"
        try:
            installed_version = pkg_resources.get_distribution(package_name).version
            # trunk-ignore(bandit/B310)
            response = urllib.request.urlopen(
                f"https://pypi.org/pypi/{package_name}/json"
            )
            json_data = json.loads(response.read())
            latest_version = json_data["info"]["version"]
            if installed_version != latest_version:
                logging.warning(
                    f"A newer version ({latest_version}) of {package_name} is available. You have version "
                    f"{installed_version}."
                )
            else:
                logging.info(
                    f"You're running the latest version of {package_name} ({latest_version})"
                )
        except pkg_resources.DistributionNotFound:
            logging.error(f"{package_name} is not installed.")
        except urllib.error.URLError as e:
            logging.warning(
                f"Could not check the latest version of {package_name} due to {type(e).__name__}: {e}"
            )

    @staticmethod
    def _handle_start_args():
        parser = argparse.ArgumentParser(
            description="All arguments are optional.",
            add_help=False,
            epilog="Please make sure that the server is already running, "
            "before you start your player.",
        )
        parser.add_argument("--help", action="help", help="Prints this help message.")
        parser.add_argument(
            "-h", "--host", help="The host to connect to. The default is 'localhost'."
        )
        parser.add_argument(
            "-p", "--port", help="The port of the host. The default is 13050.", type=int
        )
        parser.add_argument(
            "-r",
            "--reservation",
            help="Reservation code for a prepared game.",
            type=str,
        )
        parser.add_argument(
            "-R", "--room", help="Room Id the client will try to connect.", type=str
        )
        parser.add_argument(
            "-P",
            "--password",
            help="Password which will be used to authenticate with the server.",
            type=str,
        )
        parser.add_argument(
            "-s",
            "--survive",
            action="store_true",
            help="If present the client will keep running, even if the connection to the server is "
            "terminated.",
        )
        parser.add_argument(
            "-l",
            "--log",
            action="store_true",
            help="If present the client will write a log file to the current directory.",
        )
        parser.add_argument(
            "-v",
            "--verbose",
            action="store_true",
            help="Verbose option for logging.  "
            "This cancels out the log-level argument.",
        )
        parser.add_argument("-L", "--log_level", help="Sets the log level.", type=int)
        parser.add_argument(
            "--auto-reconnect",
            action="store_true",
            help="Automatically reconnect to the server if the connection is lost.",
        )
        parser.add_argument(
            "--headless",
            action="store_true",
            help="Starts the client without the penguin plugin.",
        )
        parser.add_argument(
            "-b",
            "--build",
            action="store_true",
            help="Builds the this script into a package with all its dependencies.",
        )

        parser.add_argument(
            "-d",
            "--directory",
            help="Specifies the name of the directory for the build package.",
        )

        parser.add_argument(
            "-a",
            "--architecture",
            help="Specifies the build architecture (e.g.: manylinux1_x86_64).",
        )

        return parser.parse_args()
