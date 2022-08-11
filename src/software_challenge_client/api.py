from lxml import etree
from lxml.builder import E
import socket

from src.software_challenge_client import PROTOCOL


class Messenger:
    def __init__(self, sock: socket):
        self.sock = sock

    def send_message(self, message):
        self.sock.__send(message.encode('utf-8'))

    def wait_for_message(self, buffer_size: int):
        """
        Waits for a response from the server. If the closing tag is received, then it will not wait any more and returns
        the message.
        :param buffer_size: Defines the size of the buffer, the default is set to 1024 bytes. Note that the buffer must
         be large enough to receive the complete message to avoid buffer overflow.
        :return: The XML message received, which starts and ends with the "protocol" tag.
        """
        message = self.sock.recv(buffer_size or 1024).decode('utf-8')
        while message[-7:] != PROTOCOL[-7:]:
            message += self.sock.recv(1024).decode('utf-8')
        return message


class Join:
    def __init__(self):
        self.protocol = PROTOCOL

    def join(self):
        """
        Enters any open game. If no game is open, a new one will be created automatically. Depending on the
        paused setting in server.properties the game is started paused or not.
        """
        self.protocol.append(E.join())

    def join_room(self, room_id) -> bool:
        """
        Joins a specific game that is already open but not yet started.
        :param room_id: The room id that tries to connect to. It should be opened on the server otherwise it'll be
        refused.
        :return: A boolean if the join was successful.
        """
        self.protocol.append(E.joinRoom(roomId=room_id))

        return None

    def join_prepared(self, reservation_code):
        """
        By specifying a reservation code, you can take a reserved seat in a scheduled game.
        :param reservation_code: The code which will use the server to identify the game.
        """
        self.protocol.append(E.joinPrepared(reservationCode=reservation_code))
