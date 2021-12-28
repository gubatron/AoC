import aoc
import multiprocessing
from operator import add, mul, floordiv, mod, eq

ANS1=0
ANS2=0

INP = 0
ADD = 1
MUL = 2
DIV = 3
MOD = 4
EQL = 5

instruction_codes = {
    'inp' : INP,
    'add' : ADD,
    'mul' : MUL,
    'div' : DIV,
    'mod' : MOD,
    'eql' : EQL
}

def loadProgram():
    data = aoc.readFileToStringList('24.txt')
    program = []
    for l in data:
        inst = l.split(' ')
        inst_code = instruction_codes[inst[0]]
        if inst_code == INP:
            program.append((inst_code, 'w'))
        else:
            arg1 = inst[1]
            arg2 = inst[2] if inst[2] in ('w','x','y','z') else int(inst[2])
            program.append((inst_code, (arg1, arg2)))
    return program

def isValidModelNumber(model_number, program):
    var_map = {'w' : 0, 'x': 0 , 'y':0, 'z':0}
    func_map = {
                ADD : add,
                MUL : mul,
                DIV : floordiv,
                MOD : mod,
                EQL : eq
            }
    
    model_number_list = list(map(int,str(model_number)))
    if len(model_number_list) != 14:
        return False

    for code, arg in program:
        if code == INP:
            var_map['w'] = model_number_list.pop(0)
        else:
            param1 = var_map[arg[0]]
            param2 = var_map[arg[1]] if arg[1] in var_map else arg[1]

            if code == DIV:
                if param2 == 0:
                    print('not valid - no division by zero allowed')
                    return False
            if code == MOD:
                if param1 < 0 or param2 <= 0:
                    print(f"Not valid, can't mod with a={param1}, b={param2}")
                    return False
            
            var_map[arg[0]] = func_map[code](param1, param2)
            if code == EQL:
                var_map[arg[0]] = 1 if param1 == param2 else 0
    return var_map['z'] == 0

program = loadProgram()
valid_model_numbers=[]

def scan_interval(start, end):
    print(f"scan_interval: {start}-{end}")
    tests = 0
    for r in range(start,end,-1):
        if '0' in str(r):
            continue
        if isValidModelNumber(r, program):   
            valid_model_numbers.append(r)
            print(f'Found {len(valid_model_numbers)} valid numbers, latest one is {r}')
            break
        if tests==0 or tests % 100000 == 0:
            print(f"[{start}-{end}]: tried {r}...")
        tests += 1

def generate_scan_intervals():
    END=(10**14)-1
    DELTA=11111111111110
    intervals = []
    current_interval=11111111111111
    while current_interval < END:
        current_end = current_interval+DELTA
        if current_end + DELTA > END:
            current_end = END
        intervals.append((current_interval, current_end))
        current_interval += DELTA
    return intervals

if __name__ == '__main__':
    intervals=generate_scan_intervals()
    threads=[]

    interval = intervals[0]

    for interval in intervals:        
        t = multiprocessing.Process(target=scan_interval, args=(interval[1], interval[0]), daemon=False)
        threads.append(t)
        print(f'main: started thread {t} args={interval[1]},{interval[0]}')
        t.start()

    for t in threads:
        t.join()
        print(f'Thread {t} has joined to wait for the ending of this universe')

    print("ans1={}".format(ANS1))
    print("ans2={}".format(ANS2))