import logging
import re
import socket
from typing import Union


class NetworkSocket:
    """
    This class represents a network socket that can be used to connect to a server, send data, and receive data.
    """

    def __init__(self, host="localhost", port=13050, timeout=0.1):
        """
        Initializes the NetworkSocket object with the specified host, port, and timeout values.
        The socket is not yet connected to a server.

        Args:
            host (str): The hostname or IP address of the server to connect to. Defaults to "localhost".
            port (int): The port number to connect to on the server. Defaults to 13050.
            timeout (float): The timeout for socket operations, in seconds. Defaults to 0.1.
            connected (bool): Whether the socket is currently connected to the server.
            socket (socket.socket): The underlying socket object.
            buffer (bytes): A buffer for storing received data.
        """
        self.host = host
        self.port = port
        self.timeout = timeout
        self.connected = False
        self.socket = None
        self.buffer = b""

    def connect(self):
        """
        Connects the socket to the specified host and port.
        Sets the timeout value and sets the 'connected' attribute to True.
        """
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.settimeout(self.timeout)
        self.socket.connect((self.host, self.port))
        self.connected = True

    def close(self):
        """
        Closes the socket and sets the 'connected' attribute to False.
        """
        self.socket.close()
        self.connected = False

    def send(self, data: bytes):
        """
        Sends the specified data (in bytes) to the connected server.
        """
        self.socket.sendall(data)

    def receive(self) -> Union[bytes, None]:
        """
        Attempts to receive data from the server. The received data is processed using a regular expression to extract
        complete messages. If a complete message is found, it is returned as bytes and removed from the buffer.
        If no complete message is found, None is returned.

        If a timeout occurs or a connection reset error is encountered, the socket is closed and None is returned.
        """
        regex = re.compile(
            rb"<((room[\s\S]+?</room>)|(errorpacket[\s\S]+?</errorpacket>)|(prepared[\s\S]+?</prepared>)|(joined|left|join|observe|pause|step|cancel|creatGame|authenticate)[\s\S]*?/>)"
        )
        while True:
            try:
                chunk = self.socket.recv(16129)
            except socket.timeout:
                chunk = b""
            except ConnectionResetError:
                self.close()
                return None
            if chunk:
                logging.debug(f"Received message: {chunk}")
                self.buffer += chunk
            if regex.search(self.buffer):
                receive = regex.search(self.buffer).group()
                self.buffer = self.buffer.replace(receive, b"")
                return receive
            else:
                return None
