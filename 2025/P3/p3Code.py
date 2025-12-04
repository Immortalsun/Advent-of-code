from typing import List

def find_output_joltage_p1():
    with open("p3Full.txt", "r") as file:
        accumulator = 0
        for line in file:
            cleaned_line = line.strip()
            first_digit_pos = find_largest_digit_position(cleaned_line)
            #largest digit is at end, so this becomes the ones place
            if first_digit_pos == len(cleaned_line) - 1:
                second_digit_pos = find_largest_digit_position(cleaned_line[:first_digit_pos])
                digit_str = cleaned_line[second_digit_pos]+cleaned_line[first_digit_pos]
            else:
                sub_line = cleaned_line[first_digit_pos + 1:]
                second_digit_pos = find_largest_digit_position(sub_line)
                digit_str = cleaned_line[first_digit_pos]+sub_line[second_digit_pos]

            largest_battery = int(digit_str)
            print(f"Largest battery for line {cleaned_line} is {largest_battery}")
            accumulator += largest_battery

        print(f"Total joltage is {accumulator}")

def find_output_joltage_p2():
    with open("p3Full.txt", "r") as file:
        accumulator = 0
        for line in file:
            cleaned_line = line.strip()
            used_pos: List[int] = []
            for count in range(0, 12):
                highest_pos = find_largest_digit_position_unique(cleaned_line, used_pos)
                used_pos.append(highest_pos)

            used_pos.sort()
            print(f"Used positions for line {cleaned_line} is {used_pos}")
            big_battery_str = ""
            for pos in used_pos:
                big_battery_str += cleaned_line[pos]
            print(f"Big battery string for line {cleaned_line} is {big_battery_str}")

            accumulator += int(big_battery_str)

        print(f"Total joltage is {accumulator}")

def find_largest_digit_position(input_str):
    max_digit = 0
    max_position = 0
    for i in range(0, len(input_str)):
        curr_digit = int(input_str[i])
        if curr_digit > max_digit:
            max_digit = curr_digit
            max_position = i

    return max_position


def find_largest_digit_position_unique(input_str, used_pos):
    max_digit = 0
    max_position = 0
    rem_length = 12 - len(used_pos)
    start = used_pos[len(used_pos)-1] if len(used_pos) > 0 else 0
    for i in range(start, len(input_str)):
        curr_digit = int(input_str[i])
        if i not in used_pos and len(input_str) - i >= rem_length and curr_digit > max_digit:
            max_digit = curr_digit
            max_position = i

    return max_position






if __name__ == '__main__':
    find_output_joltage_p1()
    find_output_joltage_p2()