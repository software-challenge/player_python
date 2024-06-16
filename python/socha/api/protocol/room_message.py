from socha.api.protocol.protocol_packet import ProtocolPacket


class RoomMessage(ProtocolPacket):
    """
    For all communication within a GameRoom.
    """

    ...


class RoomOrchestrationMessage(RoomMessage):
    """
    A RoomMessage that does not concern the progress of the game.
    """

    ...


class ObservableRoomMessage(RoomMessage):
    """
    A RoomMessage that can be received by observers.
    """

    ...
