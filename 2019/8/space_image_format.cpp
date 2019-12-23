#include <fstream>
#include <iostream>
#include <map>
#include <sstream>
#include <string>
#include <vector>

std::string read_program() {
  std::vector<int> program_data;
  std::ifstream infile("input.txt");
  std::string s;
  if (infile) {
    if (!getline(infile, s)) {
      return nullptr;
    }
  }
  infile.close();
  return s;
}

int BLACK = 0;
int WHITE = 1;
int TRANSPARENT = 2;

typedef struct layer {
  std::vector<std::vector<int>> sub_layers;
  int const id;
  int const width;
  int const height;

  layer(int _id, int w, int h, std::string input)
      : id(_id), width(w), height(h) {
    int offset = 0;
    int i = 0;
    while (i < input.size()) {
      // read W characters
      std::vector<int> sub_layer;
      for (int j = 0; j < width; j++) {
        int pixel = (int)((int)input[i] - 48);
        sub_layer.push_back(pixel);
        i++;
      }
      // add sublayer
      sub_layers.push_back(sub_layer);
      if (sub_layers.size() == h) {
        break;
      }
    }
  }

  int countDigits(int needle) {
    int count = 0;
    for (int i = 0; i < sub_layers.size(); i++) {
      std::vector<int> sub_layer = sub_layers[i];
      for (int j = 0; j < sub_layer.size(); j++) {
        if (sub_layer[j] == needle) {
          count++;
        }
      }
    }
    return count;
  }

  std::string toString() {
    std::ostringstream ss;
    ss << "Layer " << id << ": ";
    for (int i = 0; i < sub_layers.size(); i++) {
      if (i > 0) {
        ss << "         ";
      }

      for (int j = 0; j < sub_layers[i].size(); j++) {
        ss << sub_layers[i][j];
      }
      ss << std::endl;
    }
    ss << sub_layers.size() << " sublayers" << std::endl;
    ss << "0s => " << countDigits(0);
    return ss.str();
  }
} layer;

std::vector<layer> decode_layers(int width, int height,
                                 std::string full_input) {
  std::vector<layer> layers;
  for (int i = 0; i < full_input.size(); i += width * height) {
    layers.push_back(layer(layers.size() + 1, width, height,
                           full_input.substr(i, i + (width * height))));
  }
  return layers;
}

layer find_layer_with_fewest(int const digit,
                             std::vector<layer> const &layers) {
  int minDigitCount = 999999999;
  int result_index = 0;
  for (int i = 0; i < layers.size(); i++) {
    layer l = layers[i];
    int countedDigits = l.countDigits(digit);
    if (countedDigits < minDigitCount) {
      minDigitCount = countedDigits;
      result_index = i;
    }
  }
  return layers[result_index];
}

void part1() {
  // std::string raw_input = "123456789012";
  std::string raw_input = read_program();
  int const width = 25; // 3;
  int const height = 6; // 2;
  std::vector<layer> layers = decode_layers(width, height, raw_input);
  int digit = 0;
  layer fewest_0s = find_layer_with_fewest(digit, layers);
  std::cout << "Part I answer: " << fewest_0s.countDigits(1) << " x "
            << fewest_0s.countDigits(2) << " = "
            << (fewest_0s.countDigits(1) * fewest_0s.countDigits(2))
            << std::endl;
}

std::string xyToString(int x, int y) {
  std::ostringstream oss;
  oss << x << "," << y;
  return oss.str();
}

void print_image(std::vector<layer> const &layers, int width, int height) {
  std::map<std::string, int> image;
  int x = 0;
  int y = 0;
  int z = 0; // 0 is top layer, N grows to the bottom, covered by the top.

  for (auto it = layers.begin(); it != layers.end(); it++) {
    layer l = *it;
    for (y = 0; y < height; y++) {
      for (x = 0; x < width; x++) {
        std::string key = xyToString(x, y);
        auto it = image.find(key);
        std::vector<std::vector<int>> sub_layers = l.sub_layers;
        std::vector<int> sub_layer = sub_layers[y];
        int pixel_int = sub_layer[x];
        if (it == image.end() && pixel_int != TRANSPARENT) {
          image.insert(std::make_pair(key, pixel_int));
        }
      }
    }
  }

  // now print it:
  for (y = 0; y < height; y++) {
    for (x = 0; x < width; x++) {
      std::string key = xyToString(x, y);
      auto it = image[key];
      std::cout << it;
    }
    std::cout << std::endl;
  }
}

void part2() {
  //   std::string raw_input = "0222112222120000";
  //   int const width = 2;
  //   int const height = 2;
  std::string raw_input = read_program();
  int const width = 25;
  int const height = 6;
  std::vector<layer> layers = decode_layers(width, height, raw_input);
  std::cout << std::endl << "Part II answer:" << std::endl;
  print_image(layers, width, height);
}

int main() {
  part1();
  part2();
  return 0;
}