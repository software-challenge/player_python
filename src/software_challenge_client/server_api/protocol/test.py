from src.software_challenge_client.server_api.networking.NetworkInterface import NetworkInterface
from src.software_challenge_client.server_api.networking.PaxbStream import PaxbStream
from src.software_challenge_client.server_api.protocol.requests.LobbyRequest import JoinGameRequest

network_interface = NetworkInterface("localhost", 13050)
#networkInterface.connect()
pb_stream = PaxbStream(network_interface)
# Send a object with pb_stream 
pb_stream.send_object(JoinGameRequest("test"))
pb_stream.send_object(JoinGameRequest("test"))
pb_stream.send_object(JoinGameRequest("test"))
pb_stream.send_object(JoinGameRequest("test"))
