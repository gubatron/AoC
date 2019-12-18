#include <fstream>
#include <sstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <algorithm>

typedef struct OrbitNode {
    std::string name;
    OrbitNode* directOrbit; // parent
    std::vector< OrbitNode* > orbits;
    OrbitNode(std::string _name, OrbitNode* direct) : name(_name), directOrbit(direct) {}

    int distanceToCentralOrbit() {
      if (this->directOrbit == nullptr) {
        return 0; //COM
      }
      return 1 + this->directOrbit->distanceToCentralOrbit();
    }

    void addOrbit(OrbitNode *child) {
      auto it = std::find(this->orbits.begin(), this->orbits.end(), child);
      if (it == this->orbits.end()) {
        this->orbits.push_back(child);
      }
      if (child->directOrbit == nullptr) {
        child->directOrbit = this;
      }
    }

    OrbitNode* findOrbited(std::string nameOrbited) {
      if (this->name == nameOrbited) {
        return this;
      }
      if (this->orbits.empty()) {
        // didn't find it in this branch
        return nullptr;
      }
      auto it = this->orbits.begin();
      while (it != this->orbits.end()) {
        OrbitNode *current = *it;
        if (current->name == nameOrbited) {
          return current;
        }
        OrbitNode *candidate = nullptr;
        candidate = current->findOrbited(nameOrbited);
        if (candidate != nullptr) {
          return candidate;
        }
        it++;
      }
      return nullptr;
    }

    bool operator<(const OrbitNode &other) const {
        return this->name < other.name;
    }
} OrbitNode;

OrbitNode* findNode(std::string name, std::map<std::string, OrbitNode*> &all_nodes) {
  auto it = all_nodes.find(name);
  if (it != all_nodes.end()) {
    return it->second;
  }
  return nullptr;
}

std::map<std::string, OrbitNode*> read_program() {
  std::cout << "read_program()" << std::endl;
  std::map<std::string, OrbitNode*> all_nodes;
  OrbitNode* COM = new OrbitNode("COM", nullptr);
  std::ifstream infile("input.txt");
  while (infile) {
    std::string s;
    if (!getline(infile, s)) {
      break;
    }
    int delimiterPosition = s.find(")");
    std::string nameOrbited, nameOrbiting;
    nameOrbited = s.substr(0, delimiterPosition);
    nameOrbiting = s.substr(1+delimiterPosition, s.length());
    OrbitNode * orbited = findNode(nameOrbited, all_nodes);
    if (orbited == nullptr) {
      orbited = new OrbitNode(nameOrbited, nullptr);
    }
    OrbitNode* newOrbiter = findNode(nameOrbiting, all_nodes);
    if (newOrbiter == nullptr) {
      newOrbiter = new OrbitNode(nameOrbiting, orbited);
    }
    orbited->addOrbit(newOrbiter);
    all_nodes[nameOrbited] = orbited;
    all_nodes[nameOrbiting] = newOrbiter;
  }
  infile.close();
  return all_nodes;
}

void part1(std::map<std::string, OrbitNode*> const &all_nodes) {
  auto it=all_nodes.begin();
  int sumOrbits = 0;
  while (it != all_nodes.end()) {
    auto current = it->second;
    int distance = current->distanceToCentralOrbit();
    sumOrbits += distance;
    //std::cout << current->name << " orbits below=" << distance << " total so far=" << sumOrbits << std::endl;
    it++;
  }
  std::cout << "part 1: " << sumOrbits << std::endl;
}

std::vector<OrbitNode*> pathToCOM(OrbitNode* node) {
  std::vector<OrbitNode*> paths;
  auto parent = node->directOrbit;
  while (parent != nullptr) {
    paths.push_back(parent);
    parent = parent->directOrbit;
  }
  return paths;
}

OrbitNode* findCommonParent(std::vector<OrbitNode*> a, std::vector<OrbitNode*> b) {
  auto it_a = a.begin();
  auto it_b = b.begin();
  while (it_a != a.end()) {
    while (it_b != b.end()) {
      if (*it_a == *it_b) {
        return *it_a;
      }
      it_b++;
    }
    it_a++;
    it_b = b.begin();
  }
  return nullptr; // should not happen with our data
}

int distanceToNode(OrbitNode* source, OrbitNode* target) {
  if (source->directOrbit == target) {
    return 0;
  }
  return 1 + distanceToNode(source->directOrbit, target);
}

void part2(std::map<std::string, OrbitNode*> &all_nodes) {
  // find santa
  auto santa = findNode("SAN", all_nodes);
  // find yourself, how deep
  auto you = findNode("YOU", all_nodes);

  std::vector<OrbitNode*> santa_to_com = pathToCOM(santa);
  std::vector<OrbitNode*> you_to_com = pathToCOM(you);
  OrbitNode* common_node = findCommonParent(santa_to_com, you_to_com);
  std::cout << std::endl << "findCommonParent: found " << common_node->name << std::endl;
  int santa_distance = distanceToNode(santa, common_node);
  int your_distance = distanceToNode(you, common_node);
  std::cout << "santa_distance=" << santa_distance << std::endl;
  std::cout << "your_distance=" << your_distance << std::endl;
  std::cout << "" << std::endl;
  std::cout << "part 2: " << santa_distance << " + " << your_distance << " = " << (santa_distance+your_distance) << std::endl;
}

int main() {
  std::map<std::string, OrbitNode*> all_nodes = read_program();
  part1(all_nodes);
  part2(all_nodes);
  return 0;
}
