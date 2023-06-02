from dataclasses import dataclass
from dataclasses import field


@dataclass
class Instruction:
    syntax: str
    name: str
    mnemonic: str
    operands: dict[str, int]
    data: dict = field(default_factory=dict)
    width: int = field(default=2)

