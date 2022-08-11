import inspect
import paxb as pb
import xml.etree.ElementTree as et

from src.software_challenge_client.server_api.networking.NetworkInterface import NetworkInterface
from src.software_challenge_client.server_api.protocol.ProtocolPacket import ProtocolPacket
from src.software_challenge_client.server_api.protocol.requests.LobbyRequest import JoinGameRequest


class PaxbStream:
    def __init__(self, network_interface: NetworkInterface):
        self.network_interface = network_interface
        self.models = self.__get_models(ProtocolPacket)

    def read_object(self) -> ProtocolPacket:
        receive = self.network_interface.receive()
        xml = et.fromstring(str(receive))
        obj = pb.from_xml(self.models.get(xml.tag), str(receive))

        return obj

    def send_object(self, obj: JoinGameRequest):
        xml = pb.to_xml(obj)
        xml = "<protocol>" + xml.decode("utf-8")
        print(xml)
        self.network_interface.send(xml)

    def __get_bases(self, obj, obj_list: list) -> list:
        """
        Get the absolute parent of the object and return a list of all the children.
        :param obj: The object to get the children of.
        :param obj_list: An empty list to add the children to.
        :return: The list of children.
        """
        if isinstance(obj, (list, tuple)):
            for item in obj:
                gb = self.__get_bases(item, obj_list)
                if gb is not None:
                    obj_list.append(gb)
            return obj_list
        else:
            if "object" not in str(obj):
                if len(inspect.getmro(obj)) == 2:
                    if hasattr(obj, "__paxb_attrs__"):
                        obj_list.append(obj)

    def __get_models(self, cls: type) -> dict:
        """
        Get all the models from the children of the class.
        :param cls: The class to get the models from their children. 
        :return: A dictionary of the models.
        """
        list_bases = self.__get_bases(inspect.getmro(cls), [])

        bases = {}
        for base in list_bases:
            bases.update({base.__paxb_attrs__[0]: base})
        return bases
