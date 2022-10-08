"""
Handels the tcp connection to the server.
"""
import logging
import re
import socket
from typing import Union


class _NetworkInterface:
    """
    This interface handels all package transfers. It'll send and _receive data from a given connection.
    """

    def __init__(self, host="localhost", port=13050, timeout=5):
        """
        :param host: Host of the server. Default is localhost.
        :param port: Port of the server. Default is 13050.
        :param timeout: Timeout for receiving data from the server. Default are 10 seconds.
        """
        self.host = host
        self.port = port
        self.connected: bool = False
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.settimeout(timeout)

        self.buffer: bytes = b""

    def connect(self):
        """
        Connects the socket to the server and will be ready to listen for and send data.
        """
        self.socket.connect((self.host, self.port))
        self.connected = True
        logging.info("Connected to server.")

    def close(self):
        """
        Closes the connection to the server.
        """
        self.socket.close()
        self.connected = False
        logging.info("Closed connection.")

    def send(self, data: bytes):
        """
        Sends the data to the server. It puts the data in the sending queue and the _SocketHandler thread will get
        and send it.
        :param data: The data that is being sent as string.
        """
        self.socket.sendall(data)
        logging.debug("Sent data: %s", data.decode("utf-8"))

    def receive_socket_data(self) -> bytes:
        """
        Receives the raw tcp socket packages.
        :return: A package in bytes, None if there where no packages.
        """
        try:
            data = self.socket.recv(16129)
            # print(data.decode("utf-8"))
            return data
        except socket.timeout:
            return b""
        except ConnectionResetError:
            logging.error("The remote host closed the connection unexpectedly.")
            self.close()

    def receive(self) -> bytes:
        """
        Appends all incoming packages into one and tries to find any protocol related data.
        :return: The protocol object.
        """
        room_regex = re.compile(br"<room[\s\S]+?</room>")
        tag_regex = re.compile(br"<.*/>")
        chunk = b""
        while True:
            if chunk or self.buffer:
                self.buffer += chunk
                if room_regex.search(self.buffer):
                    receive = room_regex.search(self.buffer).group()
                    self.buffer = self.buffer.replace(receive, b"")
                    return receive
                if tag_regex.search(self.buffer):
                    receive = tag_regex.search(self.buffer).group()
                    self.buffer = self.buffer.replace(receive, b"")
                    return receive
            chunk = self.receive_socket_data()
