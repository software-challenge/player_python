import logging
import socket


class NetworkInterface:
    def __init__(self, host="localhost", port=13050, timeout=10):
        """
        NetworkInterface is a class that handles the connection to the server.
        It handles the connection between the main thread and the _SocketHandler thread.

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
        Connects the socket to the server and creates and starts the _SocketHandler thread.
        The _SocketHandler thread is a daemon thread, so it will be closed when the main thread is closed.
        """
        self.socket.connect((self.host, self.port))
        self.logger.info("Connected to server")

    def close(self):
        """
        Closes the connection to the server. The _SocketHandler thread will not be closed.
        """
        self.socket.close()
        self.logger.info("Closed connection")

    def send(self, data: str):
        """
        Sends the data to the server. It puts the data in the sending queue and the _SocketHandler thread will get
        and send it.
        :param data: The data that is being sent as string.
        """
        self.socket.sendall(data.encode("utf-8"))
        self.logger.info("Sent data: " + data)

    def receive(self) -> str:
        """
        Receives the data from the server. It gets the data from the receiving queue and returns it.
        :return: Data that is being received as string.
        """
        receiving = self.socket.recv(4096).decode("utf-8")
        return receiving
