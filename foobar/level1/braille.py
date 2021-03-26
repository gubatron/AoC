braille = {
  'a': '100000',
  'b': '110000',
  'c': '100100',
  'd': '100110',
  'e': '100010',
  'f': '110100',
  'g': '110110',
  'h': '110010',
  'i': '010100',
  'j': '010110',
  'k': '101000',
  'l': '111000',
  'm': '101100',
  'n': '101110',
  'o': '101010',
  'p': '111100',
  'q': '111110',
  'r': '111010',
  's': '011100',
  't': '011110',
  'u': '101001',
  'v': '111001',
  'w': '010111',
  'x': '101101',
  'y': '101111',
  'z': '101011',
  ' ': '000000',
}

CAP_MARK = '000001'

def replaceChar(c):
  if c != ' ' and c.isupper():
    return CAP_MARK + braille[c.lower()]
  return braille[c]

def tests():
  print(solution('code'), solution('code') == '100100101010100110100010')
  print(solution('Braille'), solution('Braille') == '000001110000111010100000010100111000111000100010')
  print(solution('The quick brown fox jumps over the lazy dog'), solution('The quick brown fox jumps over the lazy dog') == '000001011110110010100010000000111110101001010100100100101000000000110000111010101010010111101110000000110100101010101101000000010110101001101100111100011100000000101010111001100010111010000000011110110010100010000000111000100000101011101111000000100110101010110110')

def solution(s):
    return reduce(lambda a,b: a+b, map(replaceChar, s))
