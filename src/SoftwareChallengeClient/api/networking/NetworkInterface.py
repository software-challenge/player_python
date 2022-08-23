import logging
import socket
import xml.etree.ElementTree as ET
from queue import Queue


class NetworkInterface:
    """
    This interface handels all package transfers. It'll send and receive data from a given connection.
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

        self.queue = Queue()
        self.buffer: bytes = b""

    def connect(self):
        """
        Connects the socket to the server and will be ready to listen for and send data.
        """
        self.socket.connect((self.host, self.port))
        self.connected = True
        logging.info("Connected to server")

    def close(self):
        """
        Closes the connection to the server.
        """
        self.socket.close()
        self.connected = False
        logging.info("Closed connection")

    def send(self, data: bytes):
        """
        Sends the data to the server. It puts the data in the sending queue and the _SocketHandler thread will get
        and send it.
        :param data: The data that is being sent as string.
        """
        self.socket.sendall(data)
        logging.debug("Sent data: %s", data.decode("utf-8"))

    def receiveSocketData(self) -> bytes | None:
        try:
            data = self.socket.recv(8192)
            return data
        except socket.timeout:
            return None

    def receive(self) -> bytes:
        while not self.isXML(self.buffer.decode("utf-8")):
            data = self.receiveSocketData()
            if data:
                self.buffer += data.removeprefix(b"<protocol>\n  ")

        receiving = self.buffer
        self.buffer = b""
        return receiving

    @staticmethod
    def isXML(string: str) -> bool:
        try:
            ET.fromstring(string)
        except Exception:
            return False
        return True
