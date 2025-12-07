from typing import List, Tuple


def find_available_rolls_p1():
    with open("p4Full.txt", "r") as file:
        accessible_rolls = 0
        input_str_arr: List[str] = []
        for line in file:
            input_str_arr.append(line.strip())

        for y in range(0,len(input_str_arr)):
            for x in range(0,len(input_str_arr[y])):
                if input_str_arr[y][x] == "@":
                    if count_adjacencies(x,y,input_str_arr) < 4:
                        accessible_rolls += 1


        print(f"Accessible rolls: {accessible_rolls} ")


def find_available_rolls_p2():
    with open("p4Full.txt", "r") as file:
        total_rolls_removed = 0
        input_str_arr: List[str] = []
        for line in file:
            input_str_arr.append(line.strip())

        while True:
            positions_to_clear: List[Tuple[int, int]] = []
            accessible_rolls = 0
            for y in range(0,len(input_str_arr)):
                for x in range(0,len(input_str_arr[y])):
                    if input_str_arr[y][x] == "@":
                        if count_adjacencies(x,y,input_str_arr) < 4:
                            positions_to_clear.append((x,y))
                            accessible_rolls += 1

            if len(positions_to_clear) > 0:
                for (x,y) in positions_to_clear:
                    input_str_arr[y]= input_str_arr[y][:x] +  "." +input_str_arr[y][x+1:]

            if accessible_rolls == 0:
                break

            total_rolls_removed += accessible_rolls


        print(f"Total rolls removed: {total_rolls_removed} ")

def get_north(pos_x, pos_y):
    return pos_x, pos_y - 1

def get_north_east(pos_x, pos_y):
    return pos_x + 1, pos_y - 1

def get_north_west(pos_x, pos_y):
    return pos_x - 1, pos_y - 1

def get_south(pos_x, pos_y):
    return pos_x, pos_y + 1

def get_south_east(pos_x, pos_y):
    return pos_x + 1, pos_y + 1

def get_south_west(pos_x, pos_y):
    return pos_x - 1, pos_y + 1

def get_east(pos_x, pos_y):
    return pos_x + 1, pos_y

def get_west(pos_x, pos_y):
    return pos_x - 1, pos_y

def is_valid_position(pos_x, pos_y, line_len, arr_len):
    return 0 <= pos_x < line_len and 0 <= pos_y < arr_len


def count_adjacencies(start_x, start_y, input_arr):
    total_adjacencies = 0
    line_len = len(input_arr[start_y])
    arr_len = len(input_arr)
    positions: List[Tuple[int, int]] = [get_north(start_x, start_y),
         get_north_east(start_x, start_y),
         get_north_west(start_x, start_y),
         get_south(start_x, start_y),
         get_south_east(start_x, start_y),
         get_south_west(start_x, start_y),
         get_east(start_x, start_y),
         get_west(start_x, start_y)]
    for position in positions:
        if is_valid_position(position[0], position[1], line_len, arr_len):
            if input_arr[position[1]][position[0]] == "@":
                total_adjacencies += 1

    return total_adjacencies


if __name__ == '__main__':
    find_available_rolls_p1()
    find_available_rolls_p2()


