#include <array>
#include <iostream>

std::array<int, 6> to_array(int n);
// void print_int_array(std::array<int, 6> n_arr, bool append_endl);
// void print_as_int_array(int n, bool append_endl);
bool has_repeated_adjacent_digits(std::array<int, 6> n_arr);
bool has_single_pair_of_adjacent_digits(std::array<int, 6> n_arr);
bool digits_never_decrease(std::array<int, 6> n_arr);
bool is_valid_password(int n);
bool is_valid_password_2(int n);

std::array<int, 6> to_array(int n) {
  std::array<int, 6> result;
  for (int i = 5; i >= 0; i--) {
    int right_most_digit = n % 10;
    result[i] = right_most_digit;
    n /= 10;
  }
  return result;
}

// void print_int_array(std::array<int, 6> n_arr, bool append_endl) {
//   std::cout << "[";
//   for (int i = 0; i < 6; i++) {
//     std::cout << n_arr[i];
//     if (i < 5) {
//       std::cout << ", ";
//     }
//   }
//   std::cout << "]";
//   if (append_endl) {
//     std::cout << std::endl;
//   }
// }

// void print_as_int_array(int n, bool append_endl) {
//   print_int_array(to_array(n), append_endl);
// }

bool has_repeated_adjacent_digits(std::array<int, 6> n_arr) {
  for (int i = 1; i < 6; i++) {
    if (n_arr[i] == n_arr[i - 1]) {
      return true;
    }
  }
  return false;
}

bool has_single_pair_of_adjacent_digits(std::array<int, 6> arr) {
  // (11)1112
  if (arr[0] == arr[1] && 
      arr[0] != arr[2]) {
    return true;
  }
  if (arr[1] == arr[2] && 
      arr[1] != arr[3] &&
      arr[1] != arr[0]) {
    return true;
  }
  if (arr[2] == arr[3] && 
      arr[3] != arr[4] &&
      arr[2] != arr[1]) {
    return true;
  }
  if (arr[3] == arr[4] && 
      arr[3] != arr[5] &&
      arr[3] != arr[2]) {
    return true;
  }
  if (arr[4] == arr[5] && 
      arr[4] != arr[3] && 
      arr[5] != arr[3]) {
    return true;
  }
  return false;
}

bool digits_never_decrease(std::array<int, 6> n_arr) {
  for (int i = 1; i < 6; i++) {
    if (n_arr[i] < n_arr[i - 1]) {
      return false;
    }
  }
  return true;
}

bool is_valid_password(int n) {
  std::array<int, 6> n_arr = to_array(n);
  return digits_never_decrease(n_arr) && has_repeated_adjacent_digits(n_arr);
}

void part1() {
  int n_valid = 0;
  for (int i = 264793; i <= 803935; ++i) {
    if (is_valid_password(i)) {
      n_valid++;
      std::cout << n_valid << ":" << i << std::endl;
    }
  }
}

bool is_valid_password_2(int n) {
  std::array<int, 6> n_arr = to_array(n);
  return digits_never_decrease(n_arr) && has_repeated_adjacent_digits(n_arr) &&
         has_single_pair_of_adjacent_digits(n_arr);
}

void part2() {
  int n_valid = 0;
  for (int i = 264793; i <= 803935; ++i) {
    if (is_valid_password_2(i)) {
      n_valid++;
      std::cout << n_valid << ":" << i << std::endl;
    }
  }
}

int main(int n, char **args) {
  part2();
  return 0;
}