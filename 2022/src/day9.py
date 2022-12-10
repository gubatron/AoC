from typing import List, Tuple


class Pos:
    def __init__(self, x, y):
        self.x = x
        self.y = y

    def __eq__(self, other):
        return self.x == other.x and self.y == other.y

    def __hash__(self):
        return hash((self.x, self.y))

    def __repr__(self):
        return "Pos({}, {})".format(self.x, self.y)


def part1(moves: List[Tuple[int, int]]):
    start = Pos(0, 0)
    head = Pos(0, 0)
    tail = Pos(0, 0)
    visited = {}

    i = 0
    moves_len = len(moves)
    while i < moves_len:
        (dx, dy) = moves[i]

        head_moves = abs(dx)
        if dy != 0:
            head_moves = abs(dy)

        delta_x_per_move = 0
        delta_y_per_move = 0

        if dx < 0:
            delta_x_per_move = -1
        elif dx > 0:
            delta_x_per_move = 1

        if dy < 0:
            delta_y_per_move = -1
        elif dy > 0:
            delta_y_per_move = 1

        while head_moves > 0:
            # first move is always head
            head.x = head.x + delta_x_per_move
            head.y = head.y + delta_y_per_move

            # if head is in the same row as tail, and not right next to it, then we can move the tail
            if head.y == tail.y and abs(head.x - tail.x) > 1:
                # we'll move the tail towards the new location of the head - 1
                # one by one we mark the tail positions as visited
                # move to the right
                tail_dx = 0
                if head.x > tail.x:
                    tail_dx = 1
                elif head.x < tail.x:
                    tail_dx = -1
                while abs(head.x - tail.x) > 1:
                    tail = Pos(tail.x + tail_dx, tail.y)
                    update_visited(tail, visited)
            # head is in the same column as tail, and not right next to it, then we can move the tail
            elif head.x == tail.x and abs(head.y - tail.y) > 1:
                # we'll move the tail towards the new location of the head - 1
                # one by one we mark the tail positions as visited
                # move up
                tail_dy = 0
                if head.y > tail.y:
                    tail_dy = 1
                elif head.x < tail.x:
                    tail_dy = -1
                while abs(head.y - tail.y) > 1:
                    tail = Pos(tail.x, tail.y + tail_dy)
                    update_visited(tail, visited)
            # head ends up in different row(y) and column(x)
            elif head.x != tail.x and head.y != tail.y:
                # they were horizontally side by side, head moved up or down
                if abs(head.x - tail.x) == 1:
                    # head moved up more than one step
                    if head.y > tail.y and abs(head.y - tail.y) > 1:
                        # tail moves diagonally up and to the same .x of the head
                        tail = Pos(head.x, tail.y + 1)
                        update_visited(tail, visited)
                        # then tail moves up until it's one step away
                        while abs(head.y - tail.y) > 1:
                            tail = Pos(tail.x, tail.y + 1)
                            update_visited(tail, visited)
                    elif head.y < tail.y and abs(head.y - tail.y) > 1:
                        # tail moves diagonally down and to the same .x of the head
                        tail = Pos(head.x, tail.y - 1)
                        update_visited(tail, visited)
                        # then tail moves down until it's one step away
                        while abs(head.y - tail.y) > 1:
                            tail = Pos(tail.x, tail.y - 1)
                            update_visited(tail, visited)
                # they were vertically side by side, head moved left or right
                elif abs(head.y - tail.y) == 1:
                    # head moved right more than one step
                    if head.x > tail.x and abs(head.x - tail.x) > 1:
                        # tail moves diagonally right and to the same .y of the head
                        tail = Pos(tail.x + 1, head.y)
                        update_visited(tail, visited)
                        # then tail moves right until it's one step away
                        while abs(head.x - tail.x) > 1:
                            tail = Pos(tail.x + 1, tail.y)
                            update_visited(tail, visited)
                    elif head.x < tail.x and abs(head.x - tail.x) > 1:
                        # tail moves diagonally left and to the same .y of the head
                        tail = Pos(tail.x - 1, head.y)
                        update_visited(tail, visited)
                        # then tail moves left until it's one step away
                        while abs(head.x - tail.x) > 1:
                            tail = Pos(tail.x - 1, tail.y)
                            update_visited(tail, visited)

            head_moves -= 1
        i += 1
    print(f"Part 1: {count_positions_visited_once(visited)}")


def update_visited(tail, visited):
    if tail in visited:
        visited[tail] += 1
    else:
        visited[tail] = 1


def count_positions_visited_once(visited):
    return sum(1 for count in visited.values() if count == 1)


def parse_command(command):
    direction, steps = command.split()
    steps = int(steps)

    if direction == "U":
        return (0, -steps)
    elif direction == "R":
        return (steps, 0)
    elif direction == "D":
        return (0, steps)
    elif direction == "L":
        return (-steps, 0)
    else:
        raise ValueError("Unknown direction: {}".format(direction))


def readFileToStringList(path, stripped=True):
    fp = open(path, mode='r', buffering=4096)
    result = fp.readlines()
    if stripped:
        result = [s.strip() for s in result]
    fp.close()
    return result


if __name__ == "__main__":
    command_log = readFileToStringList("../9.test.txt")
    moves = []
    for command in command_log:
        moves.append(parse_command(command))
    part1(moves)
