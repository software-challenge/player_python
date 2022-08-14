from dataclasses import dataclass

import src.software_challenge_client.server_api.XStreamDecorator as XStrDec


@dataclass
@XStrDec.alias(name='slotDescriptor')
class SlotDescriptor:
    displayName: str = XStrDec.asAttribute()
    canTimeout: bool = XStrDec.asAttribute(default=True)
    reserved: bool = XStrDec.asAttribute(default=True)
