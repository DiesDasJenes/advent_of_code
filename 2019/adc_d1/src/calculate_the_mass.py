import os
from typing import List


def main_foo():
    payload_path = os.path.join(".", "data")
    print("Process file '%s'" % payload_path)
    with open(payload_path, 'r') as payload_handle:
        input_values: List[str] = payload_handle.readlines()
        calculate_fuel_requirements_easy(input_values)
        calculate_fuel_requirements_complicated(input_values)


def calculate_fuel_requirements_complicated(input_values):
    total_fuel_requirements = 0
    for value in input_values:
        fuel_requirements = do_the_math(int(value))
        while fuel_requirements >= 0:
            total_fuel_requirements += fuel_requirements
            fuel_requirements = do_the_math(fuel_requirements)
    print("COMPLICATED: The total mass is: " + str(total_fuel_requirements))


def calculate_fuel_requirements_easy(input_values: List[str]):
    fuel_requirements = 0
    for value in input_values:
        fuel_requirements += do_the_math(int(value))
    print("EASY: The total mass is: " + str(fuel_requirements))


def do_the_math(value: int) -> int:
    value = int(value / 3) - 2
    return value


if __name__ == '__main__':
    main_foo()
