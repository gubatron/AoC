#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <map>
#include <cmath>

typedef char Direction;
const Direction U = 'U';
const Direction R = 'R';
const Direction D = 'D';
const Direction L = 'L';

typedef struct WireTurn
{
  Direction direction;
  int distance;
} WireTurn;


int manhattan_distance(struct Coordinate coordinate);

class Coordinate
{
  public:
  int x;
  int y;
  explicit Coordinate() : x(0),y(0) {}
  explicit Coordinate(int _x, int _y) : x(_x), y(_y) {
  }

  std::string to_string() const {
    std::stringstream ss;
    ss << "(" << x << "," << y << ")";
    return ss.str();
  }

  bool operator<(const Coordinate &other) const {
    if (x < other.x) { return true; }
    if (x > other.x) { return false; }
    if (y < other.y) { return true; }
    return false;
  }
};

int manhattan_distance(struct Coordinate coordinate) {
  return std::abs(coordinate.x) + std::abs(coordinate.y);
}

const char EMPTY = '\0';
const char PASSED = '-';
const char CROSS = 'X';

typedef struct SlotState {
  char state;
  int steps;
  
  SlotState() : state(EMPTY), steps(0) {    
  }

  SlotState(char _state, int _steps) : state(_state), steps(_steps) {    
  }

  std::string to_string() const {
    std::stringstream ss;
    ss << "{ struct:SlotState, state:" << state << ", steps:" << steps << "}";
    return ss.str();
  } 

  bool operator<(const SlotState &other) const {
    if (steps < other.steps) return true;
    if (steps > other.steps) return false;
    if (state == EMPTY && other.state != EMPTY) return true;
    if (state == PASSED && other.state == CROSS) return true;
    if (state == PASSED && other.state == EMPTY) return false;
    return false;
  }
} SlotState;

typedef struct GridContext {
  std::map<Coordinate, SlotState> grid_1;
  std::map<Coordinate, SlotState> grid_2;
  Coordinate current_position;
  Coordinate closest_intersection;
  Coordinate fewest_intersection;
  int fewest_steps_intersection;
  explicit GridContext() : current_position(0,0), closest_intersection(100000000,100000000), fewest_steps_intersection(100000000) {}
} GridContext;

/////////////////////////////////////////////////////////////////////////////////////////////////////////
void part(int n);
void updateGrid(GridContext &context, std::vector<WireTurn> const wire, int const wire_id);
void updateSlots(GridContext &context, WireTurn turn, int const wire_id, int &steps);
WireTurn fromString(std::string wire_turn);
std::vector<std::vector<WireTurn>> read_input();
void print(WireTurn wire_turn);
void print(std::vector<WireTurn> wire_a);
/////////////////////////////////////////////////////////////////////////////////////////////////////////

int main() {
  part(2);
}

void print_keys(std::map<Coordinate, SlotState> &grid) {
  for (auto &pair : grid) {
    std::cout << pair.first.to_string() << " -> " << pair.second.to_string() << "\n";
  }
}

void part(int n) {
  GridContext context;
  std::vector<std::vector<WireTurn>> wires = read_input();
  std::vector<WireTurn> const wire_a = wires[0];
  std::vector<WireTurn> const wire_b = wires[1];
  updateGrid(context, wire_a, 0);
  std::cout << "/////////////////////////////////////////////////////////////////////////////////////////////////////////" << std::endl;
  updateGrid(context, wire_b, 1);
  std::cout << std::endl;
  std::cout << "Grid A:" << std::endl;
  print_keys(context.grid_1);
  std::cout << "/////////////////////////////////////////////////////////////////////////////////////////////////////////" << std::endl;
  std::cout << "Grid B:" << std::endl;
  print_keys(context.grid_2);
  std::cout << "/////////////////////////////////////////////////////////////////////////////////////////////////////////" << std::endl;
  if (n == 1) {
    std::cout << "The closest intersection lies at " << context.closest_intersection.to_string() << ", Manhattan Distance: " << manhattan_distance(context.closest_intersection) << std::endl << std::endl;
  } else if (n == 2) {
    std::cout << "The fewest steps intersection lies at " << context.fewest_intersection.to_string() << " : Total steps " << context.fewest_steps_intersection << std::endl;
  }
}

void updateGrid(GridContext &context, std::vector<WireTurn> const wire, int const wire_id) {
  context.current_position.x = 0;
  context.current_position.y = 0;
  int i=0;
  int steps=0;
  for (WireTurn turn : wire) {
    updateSlots(context, turn, wire_id, steps);
    i++;
  }  
}

void updateSlots(GridContext &context, WireTurn turn, int const wire_id, int &steps) {
  std::cout << "updateSlots: Turn:" << turn.direction <<  turn.distance << std::endl;  
  Coordinate target_coordinate = context.current_position;
  switch(turn.direction) {
    case U:
    target_coordinate.y += turn.distance;    
    break;
    case D:
    target_coordinate.y -= turn.distance;
    break;
    case L:
    target_coordinate.x -= turn.distance;
    break;
    case R:
    target_coordinate.x += turn.distance;
    break;
  }

  std::cout << "Current position: " << context.current_position.to_string();
  std::cout << " -> ";
  std::cout << "Target coordinate: " << target_coordinate.to_string();
  std::cout << std::endl << std::endl;

  while (((turn.direction == L || turn.direction == R)  && context.current_position.x != target_coordinate.x) ||
          ((turn.direction == U || turn.direction == D) && context.current_position.y != target_coordinate.y)) {
    
    // move one step in the right direction
    switch(turn.direction) {
      case U:
        context.current_position.y++;        
      break;
      case D:
        context.current_position.y--;
      break;
      case L:
        context.current_position.x--;
      break;
      case R:
        context.current_position.x++;
      break;
    }
    steps++;
    std::cout << "Moving... (Current position: " << context.current_position.to_string() << ", steps to get here: " << steps << ")" << std::endl;
    std::cout << std::endl;

    // C++: map.count(key) == 0 -> map.isEmptyAt(key)
    auto slot = context.grid_1.find(context.current_position);
    bool slot_empty_at_current_position = slot == context.grid_1.end();
    if (wire_id == 0) {    
      if (slot_empty_at_current_position) {
        context.grid_1[context.current_position] = SlotState(PASSED,steps);        
      }
    } else if (wire_id == 1) {            
      if (!slot_empty_at_current_position && slot->second.state == PASSED) {
        int total_steps = slot->second.steps + steps;
        context.grid_2[context.current_position] = SlotState(CROSS,steps);

        // part 1
        if (manhattan_distance(context.current_position) < manhattan_distance(context.closest_intersection)) {
          context.closest_intersection = context.current_position;
          std::cout << "New Closest Intersection at " << context.closest_intersection.to_string();
          std::cout << " Manhattan-Distance: " << manhattan_distance(context.closest_intersection) << std::endl;
        }

        // part 2
        if (total_steps < context.fewest_steps_intersection) {
          context.fewest_intersection = context.current_position;
          context.fewest_steps_intersection = total_steps;
          std::cout << "New Fewest Steps Intersection at " << context.current_position.to_string() << " : (a:" << slot->second.steps << " + b:" << steps << ") " << total_steps << std::endl;                    
        }
      } else {
        context.grid_2[context.current_position] = SlotState(PASSED,steps);
      }
    }    
  }
}

WireTurn fromString(std::string wire_turn)
{
  WireTurn result;
  result.direction = wire_turn[0];
  std::string str_distance = wire_turn.substr(1, wire_turn.size() - 1);
  std::stringstream ss(str_distance);
  ss >> result.distance;
  return result;
}

std::vector<std::vector<WireTurn>> read_input()
{
  std::vector<std::vector<WireTurn>> wire_turn_vectors;
  std::ifstream infile("input.txt");
  while (infile)
  {
    std::string s;
    if (!getline(infile, s))
    {
      break;
    }
    std::istringstream iss(s);
    std::vector<WireTurn> wire_turns;
    while (iss)
    {
      std::string s;
      if (!getline(iss, s, ','))
      {
        break;
      }
      wire_turns.push_back(fromString(s));
    }
    wire_turn_vectors.push_back(wire_turns);
  }
  infile.close();
  return wire_turn_vectors;
}

void print(WireTurn wire_turn) {
    std::cout << wire_turn.direction << " " <<  wire_turn.distance;
}

void print(std::vector<WireTurn> wire_a) {
  auto it = wire_a.begin();
  while (it != wire_a.end()) {
    print(*it);
    std::cout << std::endl;
    it++;
  }
}
