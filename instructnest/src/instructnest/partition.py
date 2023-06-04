from dataclasses import dataclass
from dataclasses import field
from typing import Callable

from instructnest.utils import compute_bitmask


@dataclass
class PartitionReport:
    name: str
    branches: set[str]
    branches_weight: dict[str, int]
    total_weight: int


@dataclass
class PartitionResult(PartitionReport):
    branches_data: dict[str, list] = field(repr=False)

    def report(self):
        print(PartitionReport.__dataclass_fields__)

class PartitionAction:
    def __init__(self, fct: Callable, name = None, store = {}):
        self.function = fct
        self.name = name or fct.__name__
        self.store = store

    def __call__(self, dataset) -> PartitionResult:
        result: dict[str, list[str]] = {}
        for item in dataset:
            inf_key = self.function(item)
            if inf_key not in result:
                result[inf_key] = []
            result[inf_key].append(item)
        return PartitionResult(
            name=self.name,
            branches=set(result),
            branches_weight={k:len(v) for k, v in result.items()},
            branches_data=result,
            total_weight=len(dataset)
        )

class PartitionActionBitmaskGenerator:
    def __init__(self, bit_map, capacity=None):
        self.bit_map = bit_map
        self.capacity = capacity

    def __call__(self, bits) -> PartitionAction:
        mch_name = "match " + compute_bitmask(bits, capacity=self.capacity)
        def partition_function(item):
            return f"{mch_name}::<{compute_bitmask([i for i in bits if item[self.bit_map[i]] == '1'], capacity=self.capacity)}>"
            # return f"{mch_name}::<{''.join([item[self.bit_map[bit]] for bit in bits])}>"

        return PartitionAction(partition_function, mch_name, {
            "used_bits": tuple(bits)
        })