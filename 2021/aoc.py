def readFileToStringList(path, stripped=True):
  fp = open(path, mode='r', buffering=4096)
  result = fp.readlines()
  if stripped:
      result = [s.strip() for s in result]
  fp.close()
  return result

def readStringsBySeparator(path, separator, stripped=True):
    stringList = readFileToStringList(path, stripped)
    bigString = ''.join(stringList)
    if not stripped:
        return bigString.split(separator)
    return [s.strip() for s in bigString.split(separator)]

def readIntList(path, stripped=False):
    stringList = readFileToStringList(path, stripped)
    return list(map(int, stringList))

if __name__ == '__main__':
    print(readStringsBySeparator('strings_by_sep_test.txt','-'))
    print(readIntList('int_list.txt'))