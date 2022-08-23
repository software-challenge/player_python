import datetime
import logging

from src.SoftwareChallengeClient.api.networking.clients.PlayerClient import PlayerClient, IClientHandler


class Starter:

    def __init__(self, host: str, port: int, logic: IClientHandler, reservation: str = None, roomId: str = None,
                 keepAlive: bool = False):
        logging.basicConfig(filename="log{}".format(datetime.datetime.now().strftime("%Y%m%d%H%M%S")),
                            level=logging.INFO)
        logging.info("Starting...")
        self.host = host
        self.port = port
        self.reservation = reservation
        self.roomId = roomId
        self.logic = logic
        self.client = PlayerClient(host=host, port=port, handler=self.logic, keepAlive=keepAlive)

        if reservation:
            self.client.joinGameWithReservation(reservation)
        elif roomId:
            self.client.joinGameRoom(roomId)
        else:
            self.client.joinGame()

        self.client.start()
