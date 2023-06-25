import json
import uuid

from pathlib import Path

from dataclasses import dataclass, field
from typing import Callable
from collections import deque
from copy import deepcopy

from instructnest.utils import dict_find
from instructnest.utils import all_d1cluters
from instructnest.partition import PartitionAction
from instructnest.partition import PartitionActionBitmaskGenerator
from instructnest.partition import PartitionResult

from functools import partial
from itertools import product
from itertools import chain


RF_BITS_MAP = dict(
    enumerate(
        chain(
            range(0,4),
            range(5,9),
            range(10,14),
            range(15,19),
        )
    )
)


def eval_trace(data: dict[str, dict]):
    if not isinstance(data, dict):
        return 0
    res = 0
    for k, v in data.items():
        res += eval_trace(v) + (1 if k.startswith("match") and "::" not in k else 0)
    return res


@dataclass
class MinimisationStrategy:
    candidate_selection: Callable
    evaluate_tracer: Callable=eval_trace


class MinimisationStrategies:
    largest = MinimisationStrategy(
        candidate_selection=lambda x: [x]
    )
    grouped = MinimisationStrategy(
        candidate_selection=all_d1cluters
    )


@dataclass
class ActionNode:
    dataset: list[str]
    deep: int
    action: PartitionAction
    bitset_used: list[int]
    trace: str

    eval_value: dict = field(default_factory=dict)

    def eval(self, owner: 'InstructionDecoderDeviceMinimisation'):
        if self.eval_value:
            return self.eval_value
        result = {}
        action_res: PartitionResult = self.action(self.dataset)

        result['balance'] = action_res.branches_weight

        forward_actions = result['forward'] = {}
        for br_name, br_data in action_res.branches_data.items():

            if len(br_data) == 1:
                result[br_name] = br_data[0]
                continue

            next_actions = owner.compute_actions(br_data, self.bitset_used)
            if not next_actions:
                result[br_name] = {"sample": br_data}
                continue

            result[br_name] = {}

            forward_actions[br_name] = [(
                ActionNode(
                    dataset=br_data,
                    deep=self.deep + 1,
                    action=x,
                    bitset_used=[*self.bitset_used, *x.store["used_bits"]],
                    trace=".".join([self.trace, self.action.name, br_name])
                )
            ) for x in next_actions]

        self.eval_value = result
        return result


@dataclass
class DecisionNode:
    actions: list[ActionNode]
    uuid: str = field(default_factory=uuid.uuid4)


class InstructionDecoderDeviceMinimisation:

    def __init__(self, bits_map = RF_BITS_MAP, strategy:MinimisationStrategy = MinimisationStrategies.grouped):
        self.rf_bits_len = len(bits_map)
        self.rf_bits_map = bits_map
        self.rf_gen = PartitionActionBitmaskGenerator(self.rf_bits_map, capacity=self.rf_bits_len-1)
        self.rf_strategy = strategy

    def compute_candidates_bits(self, dataset, skip_set=set(), print_bracker=False):
        allow_bits = set()
        breaker_hook = (
            lambda r,i, a: print(f"{i} breaker<{r[a]}>: {r}") or True
        ) if print_bracker else (
            lambda r,i, a: True
        )

        for a_bit, i_bit in self.rf_bits_map.items():
            if a_bit in skip_set:
                continue
            breaked = any(breaker_hook(item, a_bit, i_bit) for item in dataset if item[i_bit] not in "01")
            if not breaked:
                allow_bits.add(a_bit)

        return allow_bits

    def compute_actions(self, dataset, excluded=set()) -> list[PartitionAction]:
        rf_allowed = self.compute_candidates_bits(dataset, excluded)
        if not rf_allowed:
            return []

        return list(
            map(
                self.rf_gen,
                self.rf_strategy.candidate_selection(rf_allowed)
            )
        )

    def run(self, dataset: list[str], allsolutions=False):
        tr_decisions: dict[str, dict] = {}
        dataset = [i for i in dataset if not i.startswith("#")]

        st_decisions: deque[DecisionNode] = deque(
            map(
                lambda x: DecisionNode(
                    actions=[ActionNode(
                        dataset=dataset,
                        deep=1,
                        action=x,
                        bitset_used=x.store["used_bits"],
                        trace="trace"
                    )],
                ),
                self.compute_actions(dataset)
            )
        )

        while st_decisions:

            decision = st_decisions.pop()

            if decision.uuid not in tr_decisions:
                tr_decisions[decision.uuid] = {"trace": {}}
            tr_decision = tr_decisions[decision.uuid]

            fw_actions: list[dict] = []
            for action in decision.actions:
                tr_action = dict_find(tr_decision, action.trace)
                trace = action.eval(self)
                tr_action[action.action.name] = trace
                fw_action: dict[str, ActionNode] = trace.pop('forward', {})
                if fw_action:
                    fw_actions.append(fw_action)

            if not fw_actions:
                continue

            tr_decision = tr_decisions.pop(decision.uuid)

            fw_selections = list(
                product(
                    *list(
                        chain(*[
                            i.values() for i in fw_actions
                        ])
                    )
                )
            )

            for index, fw_selection in enumerate(fw_selections):
                fw_decision = DecisionNode(
                    actions=fw_selection
                )
                tr_decisions[fw_decision.uuid] = tr_decision if index == 0 else deepcopy(tr_decision)
                st_decisions.append(fw_decision)


        if allsolutions:
            return tr_decisions

        sc_decisions = {k:self.rf_strategy.evaluate_tracer(v['trace']) for k,v in tr_decisions.items()}

        best_decision = min(sc_decisions, key=sc_decisions.get)
        # Path("result.json").write_text(json.dumps(tr_decisions[best_decision], indent=2))
        return tr_decisions[best_decision]


if __name__ == "__main__":
    filepath = Path(r"D:\dev\emsgine\docs\instruction-sets\avr.opcode.nfo")
    data = filepath.read_text(encoding="utf-8").splitlines()
    solver = InstructionDecoderDeviceMinimisation(RF_BITS_MAP)
    solution = solver.run(data, True)
    Path("decoder_result.json").write_text(json.dumps(list(solution.values()), indent=2))
