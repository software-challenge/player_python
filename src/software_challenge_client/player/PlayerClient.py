from src.software_challenge_client.server_api.xflux.XFlux import XFluxClient


class IGameHandler:
    pass


class IPlayerClient:

    def joinGameWithReservation(self, reservation: str):
        ...

    def joinGameRoom(self, roomId: str):
        ...

    def joinGame(self, gameType: str = None):
        ...


class PlayerClient(IPlayerClient):

    def __init__(self, client: XFluxClient, handler: IGameHandler):
        self.client = client
        self.handler = handler
