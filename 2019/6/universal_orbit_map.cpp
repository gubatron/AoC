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
        std::cout << name << " added a new orbit " << child->name << std::endl;
      } else {
        std::cout << name << " already had orbit " << child->name << std::endl;
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

void part1() {
  std::map<std::string, OrbitNode*> all_nodes = read_program();
  auto it=all_nodes.begin();
  int sumOrbits = 0;
  while (it != all_nodes.end()) {
    auto current = it->second;
    int distance = current->distanceToCentralOrbit();
    sumOrbits += distance;
    std::cout << current->name << " orbits below=" << distance << " total so far=" << sumOrbits << std::endl;
    it++;
  }
}

int main() {
  part1();
  return 0;
}
