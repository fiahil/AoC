import re
from dataclasses import dataclass
from pathlib import Path
from typing import Dict, Generator, List


@dataclass
class Valve:
    id: str
    flow_rate: int


@dataclass
class Pipe:
    valve_from: str
    valve_to: str


@dataclass
class Network:
    valves: Dict[str, Valve]
    pipes: list[Pipe]


def parse(input_file: Path):
    capture = re.compile(r"^Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)$")
    lines = input_file.read_text().splitlines()

    valves = {}
    pipes = []
    for line in lines:
        match = capture.match(line)
        if match:
            valve = Valve(match.group(1), int(match.group(2)))
            reachable = [Pipe(valve.id, valve_to) for valve_to in match.group(3).split(", ")]
        else:
            raise ValueError(f"Invalid line: {line}")

        print(valve)
        for pipe in reachable:
            print(f"  {pipe}")

        valves[valve.id] = valve
        pipes.extend(reachable)

    print("\n---\n")

    return Network(valves, pipes)


def path_map(network: Network, minutes: int) -> Generator[List[str], None, None]:
    def need_release(current_valve: str, current_path: List[str], next: str):
        def aux(c: str, p: List[str], stop_at: List[str]):
            if c in stop_at:
                return False

            reachable = [pipe for pipe in network.pipes if pipe.valve_from == c]
            r = True if len(reachable) > 0 else False
            for pipe in reachable:
                r = r and aux(pipe.valve_to, p + [pipe.valve_to], stop_at)

                if not r:
                    break

            return r

        return aux(next, [], current_path + [current_valve])

    def aux(current_valve: str, current_path: List[str], minutes_left: int):
        if minutes_left == 0:
            yield current_path

        else:
            reachable = list(filter(lambda p: p.valve_from == current_valve, network.pipes))
            for pipe in reachable:
                has_pressure_to_release = need_release(current_valve, current_path, pipe.valve_to)
                if has_pressure_to_release:
                    yield from aux(pipe.valve_to, current_path + [pipe.valve_to], minutes_left - 1)

    yield from aux("AA", [], minutes)


def main(input_file: Path) -> int:
    # parsing input file
    n = parse(input_file)

    # creating paths
    for i, path in enumerate(path_map(n, minutes=15)):
        print(f"#{i:<4}  {path}")

    return 1651


if __name__ == "__main__":
    assert main(Path("../data/test.txt")) == 1651
