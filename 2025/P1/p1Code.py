import math

def calc_zeros_at_stops(start):
    with open("p1Full.txt", "r") as file:
        current_position = start
        zeros_counter = 0
        for line in file:
            direction = line[:1]
            distance = int(line[1:])

            if distance > 100:
                distance = distance % 100

            match direction:
                case "L":
                    current_position = current_position - distance
                case "R":
                    current_position = current_position + distance

            if current_position < 0:
                current_position = current_position + 100
            elif current_position >= 100:
                current_position = current_position - 100

            if current_position == 0:
                zeros_counter += 1

        print(zeros_counter)

def calc_zeros_at_stops_and_pass_throughs(start):
    with open("p1Full.txt", "r") as file:
        current_position = start
        starting_position = current_position
        zeros_counter = 0
        for line in file:
            direction = line[:1]
            distance = int(line[1:])

            if distance > 100:
                #account for passing through 0 on the way back to itself n times
                zeros_counter += math.floor(distance/100)
                distance = distance % 100

            match direction:
                case "L":
                    current_position = current_position - distance
                case "R":
                    current_position = current_position + distance

            if current_position < 0:
                current_position = current_position + 100
                #account for passing through 0 if we are not at 0
                #and we did not start at 0
                if current_position != 0 and starting_position != 0:
                    zeros_counter += 1
            elif current_position >= 100:
                current_position = current_position - 100
                # account for passing through 0 if we are not at 0
                # and we did not start at 0
                if current_position != 0 and starting_position != 0:
                    zeros_counter += 1

            if current_position == 0:
                zeros_counter += 1

            starting_position = current_position

        print(zeros_counter)

if __name__ == '__main__':
    calc_zeros_at_stops(50)
    calc_zeros_at_stops_and_pass_throughs(50)
