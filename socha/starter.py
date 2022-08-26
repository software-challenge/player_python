"""
This is the main entry point for the SoCha application.
"""
import datetime
import logging

from socha.api.networking.player_client import _PlayerClient, IClientHandler


class Starter:
    """
    When this is called, the client will try to connect to the server and join a game.
    When successful, the client will start the loop and call the on_update and calculate_move methods,
    if the server sends updates.
    """

    def __init__(self, host: str, port: int, logic: IClientHandler, reservation: str = None, room_id: str = None,
                 keep_alive: bool = False):
        now = datetime.datetime.now().strftime("%Y%m%d%H%M%S")
        logging.basicConfig(filename=f"log{now}", level=logging.INFO)
        logging.getLogger().addHandler(logging.StreamHandler())
        logging.info("Starting...")
        self.host = host
        self.port = port
        self.reservation = reservation
        self.roomId = room_id
        self.logic = logic
        self.client = _PlayerClient(host=host, port=port, handler=self.logic, keep_alive=keep_alive)

        if reservation:
            self.client.join_game_with_reservation(reservation)
        elif room_id:
            self.client.join_game_room(room_id)
        else:
            self.client.join_game()

        self.client.start()
