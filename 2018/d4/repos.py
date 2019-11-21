import os
import re
from datetime import datetime, timedelta


class EventBase(object):
    def __init__(self, type_: str, timestamp: datetime, guard_id: int):
        self.type = type_
        self.timestamp = timestamp
        self.guard_id = guard_id

    def __str__(self):
        return f"[{self.timestamp}] #{self.guard_id} {self.type}"


class EventGuard(EventBase):
    def __init__(self, timestamp: datetime, guard_id: int, state: str):
        super().__init__("EventGuard", timestamp, guard_id)
        self.state = state


class EventContext(EventBase):
    def __init__(self, timestamp: datetime, guard_id: int):
        super().__init__("EventContext", timestamp, guard_id)


def parse_event(event):
    d = {
        r"\[(.*)\] wakes up": lambda t, gid: EventGuard(t, gid, "awake"),
        r"\[(.*)\] falls asleep": lambda t, gid: EventGuard(t, gid, "asleep"),
        r"\[(.*)\] Guard #(\d+) begins shift": lambda t, gid: EventContext(t, gid),
    }

    for k, constructor in d.items():
        g = re.match(k, event)

        if g:
            if len(g.groups()) > 1:
                parse_event.gid = int(g.group(2))

            return constructor(
                datetime.strptime(g.group(1), r"%Y-%m-%d %H:%M"), parse_event.gid
            )

    return None


def loop(input):
    for event in input:
        yield parse_event(event)


def context_runner(state, event):
    if event.guard_id not in state["actors"].keys():
        state["actors"][event.guard_id] = {
            "asleep_at": None,
            "awake_at": None,
            "cumulated_sleeping_time": timedelta(0),
            "minutes": {m: 0 for m in range(60)},
        }
        print(f"#{event.guard_id} created")


def guard_runner(state, event):
    # switching to actor context
    own = state["actors"].get(event.guard_id)

    if not own:
        raise RuntimeError(f"Actor {event.guard_id} does not exist")

    if event.state == "asleep":
        own["asleep_at"] = event.timestamp

    if event.state == "awake":
        own["awake_at"] = event.timestamp

    # And now ε-transition
    if own["awake_at"] and own["asleep_at"]:
        own["cumulated_sleeping_time"] += own["awake_at"] - own["asleep_at"]
        print(
            f"#{event.guard_id} cumulated sleeping time : {own['cumulated_sleeping_time']}"
        )

        for m in range(int((own["awake_at"] - own["asleep_at"]).total_seconds() / 60)):
            own["minutes"][(m + own["asleep_at"].minute) % 60] += 1

        own["awake_at"] = None
        own["asleep_at"] = None


def summary(state):
    leaderboard = [
        (id, a["cumulated_sleeping_time"].total_seconds() / 60, a["minutes"])
        for id, a in state["actors"].items()
    ]

    print("= Leaderboard =")

    for e in sorted(leaderboard, key=lambda x: x[1]):
        minutes = sorted(e[2].items(), key=lambda x: -x[1])[0:5]
        print(f"#{e[0]}\t{e[1]}\t{minutes}")


def run(input):
    input = [e.strip("\n") for e in input.readlines()]
    input = loop(input)

    state = {"actors": {}}
    for event in input:
        print(event)

        if event.type == "EventContext":
            context_runner(state, event)

        if event.type == "EventGuard":
            guard_runner(state, event)

    summary(state)


if __name__ == "__main__":
    # run(open(os.path.join(os.path.dirname(__file__), "input2")))
    run(open(os.path.join(os.path.dirname(__file__), "input")))
