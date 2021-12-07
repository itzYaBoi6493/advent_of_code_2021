#!/usr/bin/python3


def solve(arr, lvl_2=False):
    min_fuel = -1
    start = min(arr)
    end = max(arr)
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

def main():
    arr = input().split(',')
    arr = list(map(lambda x: int(x), arr))

    print("Level 1: {}".format(solve(arr)))
    print("Level 2: {}".format(solve(arr, True)))

if __name__ == '__main__':
    main()
