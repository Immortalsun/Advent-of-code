from typing import List, Tuple
import math

def find_fresh_id_count_p1():
    with open("p5Full.txt", "r") as file:
        testing_ranges = False
        fresh_id_count = 0
        fresh_ranges: List[Tuple[int, int]] = []
        for line in file:
            clean_line = line.strip()
            if clean_line != "" and testing_ranges == False:
                fresh_ranges.append(get_range_tuple(clean_line))
            elif clean_line == "":
                testing_ranges = True
            elif clean_line != "" and testing_ranges == True:
                test_id = int(clean_line)
                for fresh_range in fresh_ranges:
                    if fresh_range[0] <= test_id <= fresh_range[1]:
                        fresh_id_count += 1
                        break

        print(f"Fresh id count is {fresh_id_count}")

def find_fresh_id_count_p2():
    with open("p5Full.txt", "r") as file:
        fresh_ranges: List[Tuple[int, int]] = []
        for line in file:
            clean_line = line.strip()
            if clean_line != "":
                fresh_ranges.append(get_range_tuple(clean_line))
            elif clean_line == "":
                break


        while True:
            if not try_merge_encapsulating_ranges(fresh_ranges):
                break

        fresh_id_count = 0
        for fresh_range in fresh_ranges:
            fresh_id_count += (fresh_range[1] - fresh_range[0]) + 1

        print(f"Fresh id count is {fresh_id_count}")


def get_range_tuple(input_str):
    input_items = input_str.split("-")
    bound_a = int(input_items[0])
    bound_b = int(input_items[1])
    if bound_a < bound_b:
        return bound_a, bound_b
    else:
        return bound_b, bound_a

def try_merge_encapsulating_ranges(fresh_ranges):
    for i in range(0, len(fresh_ranges)):
        for j in range(i + 1, len(fresh_ranges)):
            range_a = fresh_ranges[i]
            range_b = fresh_ranges[j]

            if range_a[0] >= range_b[0] and range_a[1] <= range_b[1]:
                fresh_ranges.pop(i)
                return True

            elif range_b[0] >= range_a[0] and range_b[1] <= range_a[1]:
                fresh_ranges.pop(j)
                return True

            elif range_a[0] <= range_b[1] and range_b[0] <= range_a[1]:
                new_range = min(range_a[0], range_b[0]), max(range_a[1], range_b[1])
                fresh_ranges[i] = new_range
                fresh_ranges.pop(j)
                return True

    return False



if __name__ == '__main__':
    #find_fresh_id_count_p1()
    find_fresh_id_count_p2()