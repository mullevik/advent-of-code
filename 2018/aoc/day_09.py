from collections import defaultdict
from collections.abc import Mapping
from dataclasses import dataclass
from typing import Self


def p1(inp: str) -> str:

    n_players, n_marbles = parse(inp)

    player_scores = play_with_linked_list(n_players, n_marbles)
    return str(max(player_scores.values()))


def p2(inp: str) -> str:
    n_players, n_marbles = parse(inp)

    player_scores = play_with_linked_list(n_players, n_marbles * 100)
    return str(max(player_scores.values()))


def play_with_array(n_players: int, n_marbles: int) -> dict[int, int]:
    marbles: list[int] = []

    p_i = 0
    curr_idx = 0
    player_scores: dict[int, int] = defaultdict(lambda: 0)
    for m_i in range(n_marbles + 1):
        if m_i > 0 and (m_i % 23) == 0:
            curr_idx = (curr_idx - 7) % len(marbles)
            taken_m = marbles.pop(curr_idx)
            player_scores[p_i] += taken_m + m_i
        else:
            curr_idx = (curr_idx + 2) % max(len(marbles), 1)
            marbles.insert(curr_idx, m_i)

        p_i = (p_i + 1) % n_players
    return dict(player_scores)


def play_with_linked_list(n_players: int, n_marbles: int) -> dict[int, int]:

    first_marble = Node(0, None, None)
    second_marble = Node(1, first_marble, first_marble)
    first_marble.next = second_marble
    first_marble.prev = second_marble
    curr_marble = first_marble
    p_i = 2
    player_scores: dict[int, int] = defaultdict(lambda: 0)
    for m_i in range(2, n_marbles + 1):
        if m_i > 0 and (m_i % 23) == 0:
            curr_marble = get_prev(curr_marble, 7)
            data_to_remove = curr_marble.data
            before_removed = remove_node(curr_marble)
            curr_marble = before_removed
            player_scores[p_i] += data_to_remove + m_i
        else:
            curr_marble = get_next(curr_marble, 1)
            curr_marble = append_to_node(curr_marble, m_i)

        p_i = (p_i + 1) % n_players
    return dict(player_scores)


@dataclass
class Node[T]:
    data: T
    prev: Self | None
    next: Self | None


def append_to_node[T](node: Node[T], data: T) -> Node[T]:
    next_node = node.next
    if next_node is None:
        raise ValueError("out of bounds")
    new_node = Node(data, node, next_node)
    node.next = new_node
    next_node.prev = new_node
    return new_node


def remove_node[T](node: Node[T]) -> Node[T]:
    prev_node = node.prev
    next_node = node.next

    if prev_node is None or next_node is None:
        raise ValueError("out of bounds")
    prev_node.next = next_node
    next_node.prev = prev_node
    return next_node


def get_next[T](node: Node[T], n: int) -> Node[T]:

    while n > 0:
        if node.next is None:
            raise ValueError("out of bounds")
        node = node.next
        n -= 1

    return node


def get_prev[T](node: Node[T], n: int) -> Node[T]:

    while n > 0:
        if node.prev is None:
            raise ValueError("out of bounds")
        node = node.prev
        n -= 1

    return node


def parse(inp: str) -> tuple[int, int]:
    parts = inp.split(" players; last marble is worth ")

    return int(parts[0]), int(parts[1].split()[0])
