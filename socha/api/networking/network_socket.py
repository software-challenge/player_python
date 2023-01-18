import socket
from typing import Union
from xml.etree import ElementTree
from xml.etree.ElementTree import XMLPullParser


class NetworkSocket:
    def __init__(self, host="localhost", port=13050, timeout=5):
        self.host = host
        self.port = port
        self.timeout = timeout
        self.connected = False
        self.socket = None
        self.buffer = b""

    def connect(self):
        self.socket = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        self.socket.settimeout(self.timeout)
        self.socket.connect((self.host, self.port))
        self.connected = True

    def close(self):
        self.socket.close()
        self.connected = False

    def send(self, data: bytes):
        self.socket.sendall(data)

    def receive(self) -> Union[bytes, None]:
        parser = XMLPullParser()
        while True:
            try:
                chunk = self.socket.recv(16129)
            except socket.timeout:
                if not self.connected:
                    self.close()
                    return None
                else:
                    continue
            except ConnectionResetError:
                self.close()
                return None

            if chunk:
                parser.feed(chunk)
                for event, elem in parser.read_events():
                    if event == "end" and (elem.tag == "room" or elem.tag == "left"):
                        return ElementTree.tostring(elem)
            else:
                return None
