#!/usr/bin/env python3
import sys

nums = [ line.strip() for line in sys.stdin ]

def find_counts(arr):
    count = [ 0 for _ in arr[0] ]
    for num in arr:
        for i in range(len(num)):
            count[i] += num[i] == '1'
    return count

def level_1(arr):
    count = find_counts(arr)
    gamma = [ 0 for _ in count ]
    epsilon = gamma[:]
    for i in range(len(count)):
        if count[i] > len(arr) // 2:
            gamma[i] = 1
        else:
            epsilon[i] = 1
    return bin_to_decimal(gamma) * bin_to_decimal(epsilon)

def level_2(arr):
    o2 = bit_criteria(arr, oxygen=True)
    co2 = bit_criteria(arr, oxygen=False)
    return o2 * co2

def bit_criteria(arr, oxygen=True):
    def flip(ch):
        if oxygen:
            return ch
        else:
            return '0' if ch == '1' else '1'
    bit = 0
    while len(arr) > 1:
        counts = find_counts(arr)
        arr = list(filter(lambda item: item[bit] == flip('1' if counts[bit] >= len(arr) - counts[bit] else '0'), arr))
        bit += 1
    return bin_to_decimal(list(map(lambda x: 1 if x == '1' else 0, arr[0])))

def bin_to_decimal(bits):
    res = 0
    for bit in bits:
        res *= 2
        res += bit
    return res

if __name__ == '__main__':
    print("Level 1: {}".format(level_1(nums)))
    print("Level 2: {}".format(level_2(nums)))
