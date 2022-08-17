import src.software_challenge_client.server_api.xflux.XFluxDecorator as XStrDec
from src.software_challenge_client.server_api.protocol import ProtocolPacket


@XStrDec.alias(name='close')
class CloseConnection(ProtocolPacket):
    """
    Is sent by one party immediately before this party closes the communication connection and should make the
    receiving party also close the connection.

    This should not be sent manually, the XFluxClient will automatically send it when stopped.
    """

    def __str__(self):
        return "CloseConnection"
