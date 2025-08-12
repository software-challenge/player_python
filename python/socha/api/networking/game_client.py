"""
This module handles the communication with the api and the students' logic.
"""

import gc
import logging
import sys
import threading
import time
from typing import List, Union

from socha._socha import GameState, Move

from socha.api.networking.xml_protocol_interface import XMLProtocolInterface
from socha.api.protocol.protocol import (
    Authenticate,
    Cancel,
    Error,
    Join,
    Joined,
    JoinPrepared,
    JoinRoom,
    Left,
    MoveRequest,
    Observe,
    Observed,
    Pause,
    Prepare,
    Prepared,
    Result,
    Room,
    Slot,
    State,
    Step,
)

from socha.api.networking.utils import handle_move, message_to_state
from socha.api.protocol.protocol import Errorpacket
from socha.api.protocol.protocol_packet import ProtocolPacket


class IClientHandler:
    history: List[List[Union[GameState, Error, Result]]] = []

    def calculate_move(self) -> Move:
        """
        Calculates a move that the logic wants the server to perform in the game room.
        """

    def on_update(self, state: GameState) -> None:
        """
        If the server _send an update on the current state of the game this method is called.
        :param state: The current state that server sent.
        """

    def on_game_over(self, roomMessage: Result) -> None:
        """
        If the game has ended the server will _send a result message.
        This method will call if this happens.

        :param roomMessage: The Result the server has sent.
        """

    def on_error(self, logMessage: str) -> None:
        """
        If error occurs,
        for instance when the logic sent a move that is not rule conform,
        the server will _send an error message and closes the connection.
        If this happens, this method is called.

        :param logMessage: The message, that server sent.
        """

    def on_room_message(self, data) -> None:
        """
        If the server sends a message that cannot be handheld by anny other method,
        this will be called.

        :param data: The data the Server sent.
        """

    def on_game_prepared(self, message) -> None:
        """
        If the game has been prepared by the server this method will be called.

        :param message: The message that server sends with the response.
        """

    def on_game_joined(self, room_id) -> None:
        """
        If the client has successfully joined a game room this method will be called.

        :param room_id: The room id the client has joined.
        """

    def on_game_left(self) -> None:
        """
        If the server left the room, this method will be called.
        If the client is running on survive mode it'll be running until shut downed manually.
        """

    def while_disconnected(self, player_client: 'GameClient') -> None:
        """
        The client loop will keep calling this method while there is no active connection to a game server.
        This can be used to do tasks after a game is finished and the server left.
        Please be aware, that the client has to be shut down manually if it is in survive mode.

        :type player_client: The player client in which the logic is integrated.
        :return: True if the client should shut down. False if the client should continue to run.
        """

    def on_create_game(self, game_client: 'GameClient') -> None:
        """
        This method will be called if the client is in admin mode and the client has authenticated with the server.
        The client can now create a game.

        :param game_client: The client that is in admin mode.
        """

    def on_prepared(
        self, game_client: 'GameClient', room_id: str, reservations: List[str]
    ) -> None:
        """
        This method will be called if the client is in admin mode and the client has created a game.

        :param game_client: The client that is in admin mode.
        :param room_id: The room id of the game.
        :param reservations: The reservations of the game.
        """

    def on_observed(self, game_client: 'GameClient', room_id: str):
        """
        This method will be called if the client is in admin mode and the client is observing a game.

        :param game_client: The client that is in admin mode.
        :param room_id: The room id of the game.
        """

    def while_waiting(self) -> None:
        """
        This method will be called while the client is waiting for the server to send a new message.
        This method is running threaded, which will be terminated without warning if the client receives a new message.
        """


class GameClient(XMLProtocolInterface):
    """
    The PlayerClient handles all incoming and outgoing objects accordingly to their types.
    """

    def __init__(
        self,
        host: str,
        port: int,
        handler: IClientHandler,
        reservation: str,
        room_id: str,
        password: str,
        auto_reconnect: bool,
        survive: bool,
        headless: bool,
    ):
        super().__init__(host, port)
        self._game_handler = handler
        self.reservation = reservation
        self.room_id = room_id
        self.password = password
        self.auto_reconnect = auto_reconnect
        self.survive = survive
        self.headless = headless

    def join_game(self):
        logging.info('Joining game')
        self.send(Join())

    def join_game_room(self, room_id: str):
        logging.info(f"Joining game room '{room_id}'")
        self.send(JoinRoom(room_id=room_id))

    def join_game_with_reservation(self, reservation: str):
        logging.info(f"Joining game with reservation '{reservation}'")
        self.send(JoinPrepared(reservation_code=reservation))

    def authenticate(self, password: str):
        logging.info(f"Authenticating with password '{password}'")
        self.send(Authenticate(password=password))

    def create_game(self, player_1: Slot, player_2: Slot, game_type: str, pause: bool):
        logging.info(
            f"Creating game with {player_1}, {player_2} and game type '{game_type}'"
        )
        self.send(Prepare(game_type=game_type, pause=pause, slot=[player_1, player_2]))

    def observe(self, room_id: str):
        logging.info(f"Observing game room '{room_id}'")
        self.send(Observe(room_id=room_id))

    def cancel(self, room_id: str):
        logging.info(f"Cancelling game room '{room_id}'")
        self.send(Cancel(room_id=room_id))

    def step(self, room_id: str):
        logging.info(f"Stepping game room '{room_id}'")
        self.send(Step(room_id=room_id))

    def pause(self, room_id: str, pause: bool):
        logging.info(f"Set pause of game room '{room_id}' to '{pause}'")
        self.send(Pause(room_id=room_id, pause=pause))

    def send_message_to_room(self, room_id: str, message):
        logging.log(15, f"Sending message to room '{room_id}'")
        logging.debug(f"Message is '{message}'")
        self.send(Room(room_id=room_id, data=message))

    def _on_object(self, message):
        """
        Process various types of messages related to a game.

        Args:
            message: The message object containing information about the game.

        Returns:
            None
        """

        if isinstance(message, Errorpacket):
            logging.error(f'An error occurred while handling the request: {message}')
            self._game_handler.on_error(str(message))
            self.stop()
        elif isinstance(message, Joined):
            logging.log(15, f"Game joined received with room id '{message.room_id}'")
            self._game_handler.on_game_joined(room_id=message.room_id)
        elif isinstance(message, Left):
            logging.log(15, f"Game left received with room id '{message.room_id}'")
            self._game_handler.on_game_left()
        elif isinstance(message, Prepared):
            logging.log(
                15, f"Game prepared received with reservation '{message.reservation}'"
            )
            self._game_handler.on_prepared(
                game_client=self,
                room_id=message.room_id,
                reservations=message.reservation,
            )
        elif isinstance(message, Observed):
            logging.log(15, f"Game observing received with room id '{message.room_id}'")
            self._game_handler.on_observed(game_client=self, room_id=message.room_id)
        elif isinstance(message, Room) and not self.headless:
            room_id = message.room_id
            if isinstance(message.data.class_binding, MoveRequest):
                logging.log(15, f"Move request received for room id '{room_id}'")
                self._on_move_request(room_id)
            elif isinstance(message.data.class_binding, State):
                logging.log(15, f"State received for room id '{room_id}'")
                self._on_state(message)
            elif isinstance(message.data.class_binding, Result):
                logging.info(f"Result received for room id '{room_id}'")
                logging.info(f"Result was '{message.data.class_binding}'")
                self._game_handler.history[-1].append(message.data.class_binding)
                self._game_handler.on_game_over(message.data.class_binding)
            else:
                logging.log(15, f"Room message received for room id '{room_id}'")
                self._game_handler.on_room_message(message.data.class_binding)
        else:
            room_id = message.room_id
            logging.log(15, f"Room message received for room id '{room_id}'")
            self._game_handler.on_room_message(message)

    def _on_move_request(self, room_id):
        start_time = time.time()
        move_response = self._game_handler.calculate_move()
        if move_response:
            response = handle_move(move_response)
            logging.info(
                f'Sent {move_response} after {round(time.time() - start_time, ndigits=3)} seconds.'
            )
            self.send_message_to_room(room_id, response)
        else:
            logging.error(f'{move_response} is not a valid move.')

    def _on_state(self, message):
        _state = message_to_state(message)
        self._game_handler.history[-1].append(_state)
        self._game_handler.on_update(_state)

    def start(self):
        """
        Starts the client loop.
        """
        self.running = True
        self._client_loop()

    def join(self) -> None:
        """
        Tries to join a game with a connected server. It uses the reservation, or room id to join.
        If they are not present it joins just without, as fallback.
        """
        if self.reservation:
            self.join_game_with_reservation(self.reservation)
        elif self.room_id:
            self.join_game_room(self.room_id)
        elif self.password:
            self.authenticate(self.password)
            self.first_time = False
            self._game_handler.on_create_game(game_client=self)
        else:
            self.join_game()

        self.first_time = False
        self._game_handler.history.append([])

    def _handle_left(self):
        self.first_time = True
        self.network_interface.close()
        if self.survive:
            logging.info(
                'The server left. Client is in survive mode and keeps running.\n'
                'Please shutdown the client manually.'
            )
            self._game_handler.while_disconnected(player_client=self)
        if self.auto_reconnect:
            logging.info('The server left. Client tries to reconnect to the server.')
            for _ in range(3):
                logging.info('Try to establish a connection with the server...')
                try:
                    self.connect()
                    if self.network_interface.connected:
                        logging.info('Reconnected to server.')
                        break
                except Exception as e:
                    logging.exception(e)
                    logging.info(
                        "The client couldn't reconnect due to a previous error."
                    )
                    self.stop()
                time.sleep(1)
            self.join()
            return
        logging.info('The server left.')
        self.stop()

    def _client_loop(self):
        """
        The client loop is the main loop, where the client waits for messages from the server
        and handles them accordingly.
        """
        while_waiting = None
        while self.running:
            if self.network_interface.connected:
                response = self._receive()
                if not response:
                    continue
                elif isinstance(response, ProtocolPacket):
                    logging.debug(f'Received new object: {response}')
                    if while_waiting:
                        while_waiting.join(timeout=0.0)
                    if isinstance(response, Left):
                        self._game_handler.on_game_left()
                        self._handle_left()
                    else:
                        self._on_object(response)
                    while_waiting = threading.Thread(
                        target=self._game_handler.while_waiting
                    )
                    while_waiting.start()
                    gc.collect()
                elif self.running:
                    logging.error(f'Received a object of unknown class: {response}')
                    raise NotImplementedError('Received object of unknown class.')
            else:
                self._game_handler.while_disconnected(player_client=self)

        logging.info('Done.')
        sys.exit(0)

    def stop(self):
        """
        Disconnects from the server and stops the client loop.
        """
        logging.info('Shutting down...')
        if self.network_interface.connected:
            self.disconnect()
        self.running = False
