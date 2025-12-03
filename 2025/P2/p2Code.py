import math

def sum_invalid_ids_p1():
    with open("p2Full.txt", "r") as file:
        input_line = file.readline()
        ranges = input_line.split(",")
        invalid_accumulator = 0
        for range_str in ranges:
            id_set = get_next_id_set(range_str)
            range_start_str = id_set[0]
            range_end_str = id_set[1]

            #test range bound strings
            if is_invalid_id_string_p1(range_start_str):
                invalid_accumulator += int(range_start_str)

            if is_invalid_id_string_p1(range_end_str):
                invalid_accumulator += int(range_end_str)

            #we have already tested the bounds, so now we
            #just test start + 1 to end - 1
            range_start_int = int(range_start_str) + 1
            range_end_int = int(range_end_str)
            for i in range(range_start_int, range_end_int):
                if is_invalid_id_int_p1(i):
                    invalid_accumulator += i

        print("Total of invalid values p1: ", invalid_accumulator)


def get_next_id_set(input_str):
    input_partition = input_str.partition("-")
    return input_partition[0], input_partition[2]

def is_invalid_id_int_p1(input_id):
    input_id_str = str(input_id)
    return is_invalid_id_string_p1(input_id_str)

def is_invalid_id_string_p1(input_id_str):
    if len(input_id_str) % 2 != 0:
        return False

    half_length = int(len(input_id_str) / 2)
    id_first_half = input_id_str[:half_length]
    id_last_half = input_id_str[half_length:]
    return int(id_first_half) == int(id_last_half)

def get_factors(input_int):
    factors = []
    for i in range(2, input_int):
        if input_int % i == 0:
            factors.append(i)
    return factors


def is_invalid_id_int_p2(input_id):
    input_id_str = str(input_id)
    return is_invalid_id_string_p2(input_id_str)

def is_invalid_id_string_p2(input_id_str):
    #make sure single digits are rejected
    if len(input_id_str) < 2:
        return False

    #check 1 digit repeats
    first_digit = int(input_id_str[0])
    single_digit_repeats = True
    for i in range(1, len(input_id_str)):
        if int(input_id_str[i]) != first_digit:
            single_digit_repeats = False
            break

    if single_digit_repeats:
        return True

    #a length 2 string should have single digit repeats if it is invalid
    #we don't need to check factors in this case
    if len(input_id_str) == 2:
        return False

    #get factorization of input length
    #for each factor, divide into equal parts*
    #and check for repeats
    factors = get_factors(int(len(input_id_str)))

    #no factors other than itself (prime)
    if len(factors) == 0:
        return False

    for factor in factors:
        all_factors_equal = True
        n_max = int(len(input_id_str) / factor)
        test_digits = int(input_id_str[:factor])

        for i in range(1, n_max):
            part = int(input_id_str[factor * i:(factor * i) + factor])
            if part != test_digits:
                all_factors_equal = False

        if all_factors_equal:
            return True

    return False


def sum_invalid_ids_p2():
    with open("p2Full.txt", "r") as file:
        input_line = file.readline()
        ranges = input_line.split(",")
        invalid_accumulator = 0
        for range_str in ranges:
            id_set = get_next_id_set(range_str)
            range_start_str = id_set[0]
            range_end_str = id_set[1]

            #test range bound strings
            if is_invalid_id_string_p2(range_start_str):
                invalid_accumulator += int(range_start_str)

            if is_invalid_id_string_p2(range_end_str):
                invalid_accumulator += int(range_end_str)

            #we have already tested the bounds, so now we
            #just test start + 1 to end - 1
            range_start_int = int(range_start_str) + 1
            range_end_int = int(range_end_str)
            for i in range(range_start_int, range_end_int):
                if is_invalid_id_int_p2(i):
                    invalid_accumulator += i

        print("Total of invalid values p2: ", invalid_accumulator)


if __name__ == '__main__':
    sum_invalid_ids_p1()
    sum_invalid_ids_p2()