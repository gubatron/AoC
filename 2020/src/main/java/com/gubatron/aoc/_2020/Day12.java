package com.gubatron.aoc._2020;

import java.io.File;
import java.io.IOException;
import java.util.List;
import java.util.stream.Collectors;

import static com.gubatron.aoc._2020.Utils.readStringsBySeparator;

public class Day12 {
    public static long part1(List<NavInstruction> instructions) {
        Ferry ferry = new Ferry();
        instructions.forEach(ferry::move);
        return ferry.manhattanDistance();
    }

    public static long part2(List<NavInstruction> instructions) {
        Ferry ferry = new Ferry();
        instructions.forEach(ferry::moveWaypoint);
        return ferry.manhattanDistance();
    }

    public static class NavInstruction {
        char action;
        int steps;

        NavInstruction(String line) {
            action = line.charAt(0);
            steps = Integer.parseInt(line.substring(1));
        }
    }

    public static class Ferry {
        int x = 0;
        int y = 0;

        // Waypoint
        int wx = 10;
        int wy = 1;

        // Waypoint relative position vector
        int w_vector_x = 10;
        int w_vector_y = 1;

        char direction = 'E';

        void move(NavInstruction inst) {
            if (inst.action == 'F') {
                if (direction == 'E') {
                    x += inst.steps;
                } else if (direction == 'W') {
                    x -= inst.steps;
                } else if (direction == 'N') {
                    y += inst.steps;
                } else if (direction == 'S') {
                    y -= inst.steps;
                }
            }

            if (inst.action == 'L') {
                if (direction == 'E') {
                    if (inst.steps == 90) {
                        direction = 'N';
                    } else if (inst.steps == 180) {
                        direction = 'W';
                    } else if (inst.steps == 270) {
                        direction = 'S';
                    }
                } else if (direction == 'W') {
                    if (inst.steps == 90) {
                        direction = 'S';
                    } else if (inst.steps == 180) {
                        direction = 'E';
                    } else if (inst.steps == 270) {
                        direction = 'N';
                    }
                } else if (direction == 'N') {
                    if (inst.steps == 90) {
                        direction = 'W';
                    } else if (inst.steps == 180) {
                        direction = 'S';
                    } else if (inst.steps == 270) {
                        direction = 'E';
                    }
                } else if (direction == 'S') {
                    if (inst.steps == 90) {
                        direction = 'E';
                    } else if (inst.steps == 180) {
                        direction = 'N';
                    } else if (inst.steps == 270) {
                        direction = 'W';
                    }
                }
            }
            if (inst.action == 'R') {
                if (direction == 'E') {
                    if (inst.steps == 90) {
                        direction = 'S';
                    } else if (inst.steps == 180) {
                        direction = 'W';
                    } else if (inst.steps == 270) {
                        direction = 'N';
                    }
                } else if (direction == 'W') {
                    if (inst.steps == 90) {
                        direction = 'N';
                    } else if (inst.steps == 180) {
                        direction = 'E';
                    } else if (inst.steps == 270) {
                        direction = 'S';
                    }
                } else if (direction == 'N') {
                    if (inst.steps == 90) {
                        direction = 'E';
                    } else if (inst.steps == 180) {
                        direction = 'S';
                    } else if (inst.steps == 270) {
                        direction = 'W';
                    }
                } else if (direction == 'S') {
                    if (inst.steps == 90) {
                        direction = 'W';
                    } else if (inst.steps == 180) {
                        direction = 'N';
                    } else if (inst.steps == 270) {
                        direction = 'E';
                    }
                }
            }

            if (inst.action == 'E') {
                x += inst.steps;
            }
            if (inst.action == 'N') {
                y += inst.steps;
            }
            if (inst.action == 'W') {
                x -= inst.steps;
            }
            if (inst.action == 'S') {
                y -= inst.steps;
            }
        }

        void moveWaypoint(NavInstruction inst) {
            if (inst.action == 'F') {
                // both ship and waypoint move
                x = x + (inst.steps * (wx - x));
                y = y + (inst.steps * (wy - y));

                // way point remains now relative to where the ship is
                wx = (x + w_vector_x);
                wy = (y + w_vector_y);
            }
            // move waypoint North
            if (inst.action == 'N') {
                w_vector_y += inst.steps;
                wy = y + w_vector_y;
            }
            // move waypoint South
            if (inst.action == 'S') {
                w_vector_y -= inst.steps;
                wy = y + w_vector_y;
            }
            // move waypoint East
            if (inst.action == 'E') {
                w_vector_x += inst.steps;
                wx = x + w_vector_x;
            }
            // move waypoint West
            if (inst.action == 'W') {
                w_vector_x -= inst.steps;
                wx = x + w_vector_x;
            }

            // Rotate waypoint Right
            if (inst.action == 'R') {
                if (inst.steps == 90) {
                    int temp = w_vector_x;
                    w_vector_x = w_vector_y;
                    w_vector_y = -temp;
                } else if (inst.steps == 180) {
                    w_vector_x = -w_vector_x;
                    w_vector_y = -w_vector_y;
                } else if (inst.steps == 270) {
                    int temp = w_vector_x;
                    w_vector_x = -w_vector_y;
                    w_vector_y = temp;
                }
                wx = x + w_vector_x;
                wy = y + w_vector_y;
            }

            // Rotate waypoint Left
            if (inst.action == 'L') {
                if (inst.steps == 90) {
                    int temp = w_vector_x;
                    w_vector_x = -w_vector_y;
                    w_vector_y = temp;
                } else if (inst.steps == 180) {
                    w_vector_x = -w_vector_x;
                    w_vector_y = -w_vector_y;
                } else if (inst.steps == 270) {
                    int temp = w_vector_x;
                    w_vector_x = w_vector_y;
                    w_vector_y = -temp;
                }
                wx = x + w_vector_x;
                wy = y + w_vector_y;
            }
        }

        int manhattanDistance() {
            return Math.abs(x) + Math.abs(y);
        }
    }

    public static void main(String[] args) throws IOException {
        //List<String> lines = readStringsBySeparator(new File("resources/sample_day_12.txt"), "\n");
        List<String> lines = readStringsBySeparator(new File("resources/input_day_12.txt"), "\n");
        List<NavInstruction> instructions = lines.stream().map(inst -> new NavInstruction(inst)).collect(Collectors.toList());

        System.out.println("DAY 12 - Rain Risk");
        System.out.println("Part 1: " + part1(instructions)); // 998
        System.out.println("==============================");
        System.out.println("Part 2: " + part2(instructions)); // 71586
    }
}
