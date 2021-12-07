#!/usr/bin/python3

arr = input().split(',')
arr = list(map(lambda x: int(x), arr))
start = min(arr)
end = max(arr)

def solve(lvl_2=False):
    min_fuel = -1
    for i in range(start, end + 1):
        fuel = 0
        for num in arr:
            val = abs(num - i)
            if lvl_2:
                val = (val * (val + 1)) // 2
            fuel += val
            if min_fuel != -1 and fuel > min_fuel:
                break
        if min_fuel == -1 or min_fuel > fuel:
            min_fuel = fuel
    return min_fuel

print("Level 1: {}".format(solve()))
print("Level 2: {}".format(solve(True)))
