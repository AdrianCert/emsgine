import logging
from dataclasses import dataclass
from dataclasses import field
from typing import Callable
from typing import TypedDict

from instructnest.common import OPERANDS_TRANSLATED_TYPES_SIZE
from instructnest.utils import all_d1cluters
from instructnest.utils import compute_bitmask


def try_cast(value: str, cls):
    try:
        return cls(value)
    except Exception:
        return value

DEDUCTIVE_FIELDS: dict[str, Callable] = {
    "name": ('syntax', lambda x: x.split(maxsplit=1)[0])
}

OPERANDS_TRANSLATED_NAMES_MAP: dict[str, str] = {
    "r": "rsr",
    "d": "rds",
    "K": "imd",
    "A": "rio",
}



class OperandsExtractResultType(TypedDict):
    operands: dict[str,list[int]]
    extractors: dict[str, list[tuple[str, int]]]
    masks: dict[str, str]
    types: dict[str, str]


@dataclass
class InstructionPattern:
    pattern: str
    syntax: str
    name: str
    data: dict = field(default_factory=dict)
    width: int = field(default=2)

    @classmethod
    def fromstr(cls, data: str) -> 'InstructionPattern':
        pattern, *pairs = data.split(" $")
        data_dict = {}
        data_fields = {}
        annt = cls.__annotations__
        for pair in pairs:
            if " " not in pair:
                continue
            pair_items = pair.split(maxsplit=1)
            if len(pair_items) == 1:
                logging.debug(f"Ignored pair:`{pair}` on `{data}`")
                continue

            key, value = pair_items
            data_dict[key] = value
            if key in annt:
                data_fields[key] = try_cast(value, annt[key])
        for field in annt:
            if field in ["data", "pattern"]:
                continue
            if field in data_fields:
                continue
            d_field, d_proc = DEDUCTIVE_FIELDS.get(field)
            r_field = d_proc(data_dict[d_field])
            data_fields[field] = r_field
            data_dict[field] = r_field
        return cls(pattern=pattern, data=data_dict, **data_fields)

    def compute_operands(self):
        operands: dict[str, list[int]] = {}
        for index, value in enumerate(filter(lambda x: x!=" ", self.pattern)):
            if value in "01":
                continue
            if value not in operands:
                operands[value] = []
            operands[value].append(index)

        return operands

    def compute_operands_extract(self, word_size) -> OperandsExtractResultType:
        ops = self.compute_operands()
        exr = {
            "operands": ops,
            "extractors": {
                compute_bitmask(v):dict(
                    components=list(BitcodedExtractorGenerator.frombitset(v, word_size)),
                    capacity=len(ops[k])
                ) for k,v in ops.items()
            },
            "masks": {
                k:compute_bitmask(v) for k,v in ops.items()
            },
            "types": {
                k:self.data.get(f"op_type.{k}") or (
                    OPERANDS_TRANSLATED_TYPES_SIZE.get(len(ops[k]) // 8 + 1) or "u8"
                ) for k in ops
            }
        }
        return exr


class BitcodedExtractorGenerator:

    @staticmethod
    def frombitset(bitset, word_size=2):
        b_overview: dict[int, list[int]]= {}
        for b_clt in all_d1cluters(bitset):
            b_max = max(b_clt)
            b_overview[b_max] = b_clt

        bmsk_capacity = 8 * word_size - 1
        lsb_cursor = 0
        for b_comp_key in sorted(b_overview.keys(), reverse=True):
            b_comp_bits = b_overview[b_comp_key]
            words_parts = BitcodedExtractorGenerator.partition_word_bits(b_comp_bits, word_size)
            for b_comp_word in sorted(words_parts.keys(), reverse=True):
                b_comp_bset: list[int] = words_parts[b_comp_word]
                b_comp_lead: int = max(b_comp_bset)
                b_comp_mask: str = compute_bitmask(b_comp_bset, capacity=bmsk_capacity)
                b_comp_dist: int = bmsk_capacity - lsb_cursor - b_comp_lead
                lsb_cursor += len(words_parts[b_comp_word])
                yield b_comp_mask, b_comp_dist, b_comp_word

    @staticmethod
    def partition_word_bits(bitset, word_size=2):
        words: dict[int, list[int]] = {}
        bcount = 8 * word_size
        for bit in bitset:
            word, nbit = divmod(bit, bcount)
            if word not in words:
                words[word] = []
            words[word].append(nbit)
        return words
