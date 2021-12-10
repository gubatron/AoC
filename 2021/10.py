import aoc

lines = aoc.readFileToStringList("10.txt")


def handleChar(c, char_stack, illegal_stack):
    matching_opening = {')': '(', ']': '[', '}': '{', '>': '<'}
    score = {')': 3, ']': 57, '}': 1197, '>': 25137}
    if c in ['(', '[', '{', '<']:
        char_stack.append(c)
    if c in [')', ']', '}', '>']:
        if len(char_stack) == 0:
            illegal_stack.append(c)
            return score[c]
        else:
            if char_stack[-1] == matching_opening[c]:
                char_stack.pop()
            else:
                illegal_stack.append(c)
                return score[c]
    return 0


def process_line(line, illegal_stack):
    expression_stack = []
    for c in line:
        score = handleChar(c, expression_stack, illegal_stack)
        if score > 0:
            return score
    if len(expression_stack) > 0:
        return -1  # incomplete
    return 0


def complete(line):
    opening = {'(', '[', '{', '<'}
    matching_closing = {'(': ')', '[': ']', '{': '}', '<': '>'}
    stack = []
    completion = []

    # pop what you can
    for c in line:
        if c in opening:
            stack.append(c)
        else:
            last = stack.pop()
    # complete the expression
    while len(stack) > 0:
        completion.append(matching_closing[stack.pop()])
    return completion


ln = 0
total_score = 0
incomplete = []
for l in lines:
    illegals = []
    score = process_line(l, illegals)
    if score > 0:
        total_score += score
    elif score < 0:
        incomplete.append(l)
    ln += 1
ANS1 = total_score


def scoreCompletion(completion):
    score = 0
    completion_scores = {')': 1, ']': 2, '}': 3, '>': 4}
    for c in completion:
        score = score * 5 + completion_scores[c]
    return score


autocompletion_scores = []
for l in incomplete:
    completion = complete(l)
    score = scoreCompletion(completion)
    autocompletion_scores.append(score)

autocompletion_scores = sorted(autocompletion_scores)
middle = int(len(autocompletion_scores) / 2)
ANS2 = autocompletion_scores[middle]

print("ans1={}".format(ANS1))  # 436497
print("ans2={}".format(ANS2)) # 2377613374