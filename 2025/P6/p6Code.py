from typing import List


def problem_summation_p1():
    with open("p6Full.txt", "r") as file:
        numbers_arr: List[List[int]] = []
        op_arr: List[str] = []
        for line in file:
            clean_line = line.strip()
            if clean_line.startswith("+") or clean_line.startswith("*"):
                op_arr = get_operators_array(clean_line)
            else:
                numbers_arr.append(get_num_array(clean_line))

        summation = 0
        for i in range(0, len(op_arr)):
            match op_arr[i]:
                case "+":
                    prob_solution = 0
                    for num_arr in numbers_arr:
                        prob_solution += num_arr[i]
                case "*":
                    prob_solution = 1
                    for num_arr in numbers_arr:
                        prob_solution *= num_arr[i]

            summation += prob_solution

        print(f"Summation for all problems is: {summation}")

def problem_summation_p2():
    with open("p6Full.txt", "r") as file:
        input_str_arr: List[str] = []
        for line in file:
            clean_line = line.replace("\n", "")
            input_str_arr.append(clean_line)

        char_pos = len(input_str_arr[0]) - 1
        numbers_arr: List[int] = []
        summation = 0
        while char_pos >= 0:
            num_str = ""
            for i in range(0, len(input_str_arr) - 1):
                if input_str_arr[i][char_pos].isdigit():
                    num_str += input_str_arr[i][char_pos]
            
            if num_str != "":
                numbers_arr.append(int(num_str))

            if input_str_arr[len(input_str_arr) - 1][char_pos] == "+" or input_str_arr[len(input_str_arr) - 1][char_pos] == "*":
                op = input_str_arr[len(input_str_arr) - 1][char_pos]
                match op:
                    case "+":
                        prob_solution = 0
                        for num in numbers_arr:
                            prob_solution += num
                    case "*":
                        prob_solution = 1
                        for num in numbers_arr:
                            prob_solution *= num

                numbers_arr.clear()
                summation += prob_solution
            
            char_pos -= 1

        print(f"Summation for all problems is: {summation}")




def get_num_array(input_string):
    curr_num_str = ""
    num_array = []
    for char in input_string:
        if char.isdigit():
            curr_num_str += char
        elif curr_num_str != "":
            num_array.append(int(curr_num_str))
            curr_num_str = ""

    if curr_num_str != "":
        num_array.append(int(curr_num_str))

    return num_array

def get_operators_array(input_string):
    curr_operator_str = ""
    op_array = []
    for char in input_string:
        if char == "+" or char == "*":
            curr_operator_str += char
        elif curr_operator_str != "":
            op_array.append(curr_operator_str)
            curr_operator_str = ""

    if curr_operator_str != "":
        op_array.append(curr_operator_str)

    return op_array



if __name__ == '__main__':
    #problem_summation_p1()
    problem_summation_p2()