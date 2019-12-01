import os
import unittest
from typing import List

import calculate_the_mass


class TestCalculateTheMass(unittest.TestCase):

    def test_should_divide_by_three_and_round_down_and_substract_2(self):
        self.assertEqual(2, calculate_the_mass.calculate_fuel_requirements_easy("12"))
        self.assertEqual(2, calculate_the_mass.calculate_fuel_requirements_easy("14"))
        self.assertEqual(654, calculate_the_mass.calculate_fuel_requirements_easy("1969"))
        self.assertEqual(33583, calculate_the_mass.calculate_fuel_requirements_easy("100756"))

    def test_should_calculate_fuel_requirements_easy(self):
        masses = ["12", "14"]
        self.assertEqual(4, calculate_the_mass.calculate_fuel_requirements_easy(masses))

    def test_should_calculate_fuel_requirements_complicated(self):
        masses = ["100756"]
        self.assertEqual(50346, calculate_the_mass.calculate_fuel_requirements_complicated(masses))

if __name__ == '__main__':
    unittest.main()
