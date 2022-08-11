import queue
import logging
import socket
import threading
from _queue import Empty
from time import sleep
from dataclasses import dataclass


@dataclass
class _Receiving:
    data: ...


@dataclass
class _Sending:
    data: ...


class _DataStream:
    def __init__(self):
        """
        _DataStream is a class that stores the data that is being sent and received.
        For that it uses two queues. One for sending and one for receiving.
        This is used to communicate with the main thread and the _SocketHandler thread.
        """
        self.dataQueue = queue.Queue()
        self.received_data_queue = queue.Queue()
        self.sending_data_queue = queue.Queue()
        self.logger = logging.getLogger(__name__)

    def putSendingData(self, data):
        """
        Puts data in the sending queue.
        :param data: The data that is being sent.
        """
        self.logger.info("Putting data in the queue")
        data = _Sending(data)
        try:
            self.dataQueue.put(data, block=False)
        except Empty:
            return None

    def getSendingData(self):
        """
        Gets data from the sending queue.
        :return: The data that is being sent. Returns None if the queue is empty.
        """
        self.logger.info("Getting data from the queue")
        item = self.dataQueue.get(block=False)
        while type(item) is not _Sending:
            item = self.dataQueue.get(block=False)
        return item

    def putReceivingData(self, data):
        """
        Puts data in the receiving queue.
        :param data: The data that is being received.
        """
        self.logger.info("Putting data in the queue")
        data = _Receiving(data)
        try:
            self.dataQueue.put(data, block=False)
        except Empty:
            return None

    def getReceivingData(self):
        """
        Gets data from the receiving queue.
        :return: The data that is being received. Returns None if the queue is empty.
        """
        self.logger.info("Getting data from the queue")
        while not self.isReceiverQueue():
            ...
        return self.dataQueue.get(block=False)

    def isReceiverQueue(self) -> bool:
        """
        Checks if the queue has data in it.
        :return: False if the queue is empty, True if the queue is not empty.
        """
        item = self.dataQueue.get_nowait()
        if type(item) is _Receiving:
            return True
        else:
            self.dataQueue.put_nowait(item)
            return False

    def isSenderQueue(self) -> bool:
        """
        Checks if the queue has data in it.
        :return: False if the queue is empty, True if the queue is not empty.
        """
        item = self.dataQueue.get_nowait()
        if type(item) is _Sending:
            return True
        else:
            self.dataQueue.put_nowait(item)
            return False


class _SocketHandler(threading.Thread):
    def __init__(self, skt: socket, storage: _DataStream):
        """
        _SocketHandler is a class that handles the socket incoming and outgoing data.
        :param skt: The socket that is being used.
        :param storage: The data storage that has the sending and receiving queues.
        """
        threading.Thread.__init__(self)
        self.socket = skt
        self.data_storage = storage
        self.logger = logging.getLogger(__name__)

    def run(self):
        """
        The run method override. It's checks if the socket received data and puts it in the receiving queue.
        Furthermore it checks if the sending queue is not empty and sends the data if so.
        """
        self.logger.info("Starting socket listener")
        while True:
            if self.data_storage.isSenderQueue():
                self.logger.info("Sender queue has data")
                data = self.data_storage.getSendingData()
                self.__send(data)
            data = self.__receive()
            if len(data) > 0:
                self.logger.info("Received data: " + data.decode("utf-8"))
                if "</protocol>" in data.decode("utf-8"):
                    self.logger.info("Received </protocol>")
                    break
                self.data_storage.putReceivingData(data)

    def __send(self, data):
        """
        Sends the data to the server.
        :param data: The data that is being sent. The data has not to be bytes, because they will encoded to bytes.
        """
        self.socket.sendall(data.encode())
        self.logger.info("Sent data: " + data)

    def __receive(self):
        """
        Receives the data from the server.
        :return: The data that is being received. Returns None if the socket is closed. Returns an empty string if there is no data.
        """
        data = self.socket.recv(1024)
        return data


class NetworkInterface:
    def __init__(self, host, port):
        """
        NetworkInterface is a class that handles the connection to the server.
        It handles the connection between the main thread and the _SocketHandler thread.
        :param host: The host of the server. Fallback is localhost.
        :param port: The port of the server. Fallback is 13050.
        """
        self.host = host or "localhost"
        self.port = port or 13050
        self.socketHandlerThread = None
        self.dataStorage = _DataStream()
        self.logger = logging.getLogger(__name__)
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)

    def connect(self):
        """
        Connects the socket to the server and creates and starts the _SocketHandler thread.
        The _SocketHandler thread is a daemon thread, so it will be closed when the main thread is closed.
        """
        self.socket.connect((self.host, self.port))
        sleep(1)
        self.socketHandlerThread = _SocketHandler(self.socket, self.dataStorage)
        self.socketHandlerThread.daemon = True
        self.socketHandlerThread.start()
        self.logger.info("Connected to server")

    def close(self):
        """
        Closes the connection to the server. The _SocketHandler thread will not be closed.
        """
        self.socket.close()
        self.logger.info("Closed connection")

    def send(self, data):
        """
        Sends the data to the server. It puts the data in the sending queue and the _SocketHandler thread will get
        and send it.
        :param data: The data that is being sent.
        """
        self.dataStorage.putSendingData(data)
        self.logger.info("Sending data...")

    def receive(self):
        """
        Receives the data from the server. It gets the data from the receiving queue and returns it.
        :return: The data that is being received. Returns None if the queue is empty.
        """
        while not self.dataStorage.isReceiverQueue():
            ...

        receiving = self.dataStorage.getReceivingData()
        self.logger.info("Received data...")
        return receiving
