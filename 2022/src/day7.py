def readFileToStringList(path, stripped=True):
    fp = open(path, mode='r', buffering=4096)
    result = fp.readlines()
    if stripped:
        result = [s.strip() for s in result]
    fp.close()
    return result

class File:
    def __init__(self, name, size):
        self.name = name
        self.size = size

class Dir:
    def __init__(self, name, parent=None):
        self.name = name
        self.files = []
        self.subdirs = []
        self.parent = parent

    def add_file(self, file):
        self.files.append(file)

    def try_add_dir_by_name(self, name):
        for subdir in self.subdirs:
            if subdir.name == name:
                return subdir
        new_dir = Dir(name, self)
        self.subdirs.append(new_dir)
        return new_dir

    def get_size(self):
        size = 0
        for file in self.files:
            size += file.size
        for subdir in self.subdirs:
            size += subdir.get_size()
        return size

    def get_dir_sizes(self):
        result = []
        result.append(self.get_size())
        for subdir in self.subdirs:
            result += subdir.get_dir_sizes()
        return result

    def depth(self):
        if self.parent is None:
            return 0
        return self.parent.depth() + 1

    def print(self, indent=0):
        print(' ' * indent, end='')
        print(f'- {self.name} (dir) total size: {self.get_size()}')

        for folder in self.subdirs:
            folder.print(indent + 1)

        for file in self.files:
            print(' ' * (self.depth() + 1),end='')
            print('- ' + file.name + ' (file, size=' + str(file.size) + ')')

    def get_directories_with_size_up_to(self, size):
        result = []
        if self.get_size() <= size:
            result.append(self)
        for subdir in self.subdirs:
            result += subdir.get_directories_with_size_up_to(size)
        return result

if __name__ == '__main__':
    lines = readFileToStringList('../7.txt')
    root = Dir('/')
    current_dir = root
    for line in lines:
        print(line)
        if line.startswith('$ cd ') and not line.endswith('/'):
            dir_name = line.split('$ cd ')[1]
            if dir_name == '..' and current_dir.parent is not None:
                current_dir = current_dir.parent
            else:
                # ads the new dir if it didn't have it, otherwise returns the existing one
                current_dir = current_dir.try_add_dir_by_name(dir_name)
            print(f'-> current directory is: {current_dir.name}')
        if line.startswith('$ ls'):
            continue
        if line.startswith('dir'):
            dir_name = line.split('dir ')[1]
            current_dir.try_add_dir_by_name(dir_name)
        elif line[0].isdigit():
            file_size, file_name  = line.split(' ')
            current_dir.add_file(File(file_name, int(file_size)))
    print()
    root.print()
    winners = root.get_directories_with_size_up_to(100000)
    print("part1: " + str(sum(list(map(lambda x: x.get_size(), winners)))))


    # part 2
    print("dir sizes: ")

    disk_space = 70000000
    space_used = root.get_size()
    free_space = disk_space - space_used
    space_needed = 30000000 - free_space
    folder_sizes = root.get_dir_sizes()
    folder_sizes.sort()
    for size in folder_sizes:
        if size > space_needed:
            print("part2: " + str(size))
            break

