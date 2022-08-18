from src.SoftwareChallengeClient.player.Starter import ILogic
from src.SoftwareChallengeClient.player.Starter import Starter
from src.SoftwareChallengeClient.server_api.sc.api.plugins.IPlugins import IGameState

Starter("Localhost", 13050)


class Logic(ILogic):

    def onUpdate(self, state: IGameState):
        print("Test")
