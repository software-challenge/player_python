import socket
import unittest
from unittest.mock import MagicMock

from socha.api.networking.network_socket import NetworkSocket


class ReceiveTestCase(unittest.TestCase):
    def setUp(self):
        self.my_obj = NetworkSocket()
        self.my_obj.socket = MagicMock()
        self.my_obj.close = MagicMock()

    def test_receive_with_complete_message(self):
        self.my_obj.socket.recv.return_value = b"<room>Test message</room>"
        result = self.my_obj.receive()
        self.assertEqual(result, b"<room>Test message</room>")

    def test_receive_with_incomplete_message(self):
        self.my_obj.socket.recv.return_value = b"<room>Test message"
        result = self.my_obj.receive()
        self.assertIsNone(result)

    def test_receive_with_timeout(self):
        self.my_obj.socket.recv.side_effect = socket.timeout
        result = self.my_obj.receive()
        self.assertIsNone(result)

    def test_receive_with_connection_reset_error(self):
        self.my_obj.socket.recv.side_effect = ConnectionResetError
        result = self.my_obj.receive()
        self.assertIsNone(result)
        self.my_obj.close.assert_called_once()
        self.assertIsNone(result)
