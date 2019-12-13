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

  std::string to_string() {
    std::stringstream ss;
    ss << "(" << x << "," << y << ")";
    return ss.str();
  }
};

int manhattan_distance(struct Coordinate coordinate) {
  return std::abs(coordinate.x) + std::abs(coordinate.y);
}
typedef char SlotState;
const SlotState EMPTY = '\0';
const SlotState PASSED = '-';
const SlotState CROSS = 'X';

typedef struct GridContext {
  std::map<std::string, SlotState> grid_1;
  std::map<std::string, SlotState> grid_2;
  Coordinate current_position;
  Coordinate closest_intersection;
  explicit GridContext() : current_position(0,0), closest_intersection(100000000,100000000) {}
} GridContext;

/////////////////////////////////////////////////////////////////////////////////////////////////////////
void part1();
void updateGrid(GridContext &context, std::vector<WireTurn> const wire, int const wire_id);
void updateSlots(GridContext &context, WireTurn turn, int const wire_id);
WireTurn fromString(std::string wire_turn);
std::vector<std::vector<WireTurn>> read_input();
void print(WireTurn wire_turn);
void print(std::vector<WireTurn> wire_a);
/////////////////////////////////////////////////////////////////////////////////////////////////////////


int main() {
  part1();
}

void print_keys(std::map<std::string, SlotState> &grid) {
  for (auto const& pair : grid) {
    std::cout << pair.first << " -> " << pair.second << "\n";
  }
}

void part1() {
  GridContext context;
  std::vector<std::vector<WireTurn>> wires = read_input();
  std::vector<WireTurn> const wire_a = wires[0];
  std::vector<WireTurn> const wire_b = wires[1];
  updateGrid(context, wire_a, 1);
  std::cout << "/////////////////////////////////////////////////////////////////////////////////////////////////////////" << std::endl;
  updateGrid(context, wire_b, 2);
  std::cout << std::endl;
  std::cout << "The closest intersection lies at " << context.closest_intersection.to_string() << ", Manhattan Distance: " << manhattan_distance(context.closest_intersection) << std::endl << std::endl;
}

void updateGrid(GridContext &context, std::vector<WireTurn> const wire, int const wire_id) {
  context.current_position.x = 0;
  context.current_position.y = 0;
  int i=0;
  for (WireTurn turn : wire) {
    updateSlots(context, turn, wire_id);    
    i++;
  }  
}

void updateSlots(GridContext &context, WireTurn turn, int const wire_id) {
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

    std::cout << "Moving... (Current position: " << context.current_position.to_string() << ")";
    std::cout << std::endl;

    // C++: map.count(key) == 0 -> map.isEmptyAt(key)
    if (wire_id == 1) {
      std::cout << "Grid1 @ " << context.current_position.to_string() << context.grid_1[context.current_position.to_string()] << std::endl;
      if (context.grid_1[context.current_position.to_string()] == EMPTY) {
        context.grid_1[context.current_position.to_string()] = PASSED; 
        std::cout << "Grid1 @ " << context.current_position.to_string();
        std::cout << " marked as PASSED" << std::endl;
      } else {
        std::cout << "Grid1 @ " << context.current_position.to_string() << " had something: [" <<  context.grid_1[context.current_position.to_string()] << "] count:" << context.grid_1.count(context.current_position.to_string())  << std::endl;
      }
    } else if (wire_id == 2) {      
      std::cout << "Wire B: ";
      if (context.grid_1[context.current_position.to_string()] == PASSED) {        
        context.grid_2[context.current_position.to_string()] = CROSS;      
        if (manhattan_distance(context.current_position) < manhattan_distance(context.closest_intersection)) {
          context.closest_intersection = context.current_position;
          std::cout << "New Closest Intersection at " << context.closest_intersection.to_string();
          std::cout << " Manhattan-Distance: " << manhattan_distance(context.closest_intersection) << std::endl;
        }
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
