import logging
import socket


class NetworkInterface:
    """
    This interface handels all package transfers. It'll send and receive data from a given connection.
    """

    def __init__(self, host="localhost", port=13050, timeout=10):
        """
        :param host: Host of the server. Default is localhost.
        :param port: Port of the server. Default is 13050.
        :param timeout: Timeout for receiving data from the server. Default are 10 seconds.
        """
        self.host = host
        self.port = port
        self.logger = logging.getLogger(__name__)
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.settimeout(timeout)

    def connect(self):
        """
        Connects the socket to the server and will be ready to listen for and send data.
        """
        self.socket.connect((self.host, self.port))
        self.logger.info("Connected to server")

    def close(self):
        """
        Closes the connection to the server.
        """
        self.socket.close()
        self.logger.info("Closed connection")

    def send(self, data: bytes):
        """
        Sends the data to the server. It puts the data in the sending queue and the _SocketHandler thread will get
        and send it.
        :param data: The data that is being sent as string.
        """
        self.socket.sendall(data)
        self.logger.info("Sent data: ", data)

    def receive(self) -> bytes:
        """
        Receives the data from the server.
        :return: Data that is being received as string.
        """
        receiving = self.socket.recv(4096)
        return receiving
