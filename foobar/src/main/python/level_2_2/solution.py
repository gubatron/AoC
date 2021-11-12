def solution(xs):
    positives = list(filter(lambda n: n > 0 and abs(n) <= 1000, xs))
    negatives = list(filter(lambda n: n < 0 and abs(n) <= 1000, xs))
    power = 1
    if not positives and not negatives:
        return "0"
    if not positives and 0 not in xs and len(negatives) == 1:
        return str(negatives[0])
    if not positives and 0 in xs and len(negatives) == 1:
        return "0"
    if len(negatives) % 2 == 1 and len(negatives) > 1:
        negatives.remove(max(negatives))
    if positives:
        for x in positives:
            power = power * x
    if negatives:
        for x in negatives:
            power = power * x
    return str(power)

cases = [([0], 0),  # 0
         ([-1], -1),  # 1
         ([-4], -4),  # 2
         ([10], 10),  # 3
         ([-1, 0], 0),  # 4!
         ([-4, 0], 0),  # 5
         ([-4, 0, -1], 4),  # 6
         ([-4, 0, -3, -2], 12),  # 7
         ([-2, -1, -1, 0], 2),  # 8
         ([1], 1),  # 9
         ([-1, -1], 1),  # 10
         ([-1, 0, -1], 1),  # 11
         ([-1, -1, 1], 1),
         ([-2, -1, -1, 1], 2),
         ([-2, -1, -1, 0, 1], 2),
         ([-2, -1, -1, 0, 1, 2, -6], 24),
         ([2, -3, 1, 0, -5], 30),
         ([2, 0, 2, 2, 0], 8),
         ([-2, -3, 4, -5], 60),
         ([2, 0, 2, 2, 0], 8),
         ([0, 0, 0, -43], 0),
         ([0, 0], 0),
         ([0, 0, 4], 4)
         ]
n = 0
for case in cases:
    sol = solution(case[0])
    if sol != str(case[1]):
        print(n, case[0], solution(case[0]), str(case[1]))
    n += 1
