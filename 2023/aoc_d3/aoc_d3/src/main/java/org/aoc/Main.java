package org.aoc;

import java.nio.file.Paths;

public class Main {
  public static void main(String[] args) {
    char[][] grid =
        AoCInputFileReader.readFileFToTwoDimensionalArrayFrom(
            Paths.get("src/main/resources/puzzle-input.txt").toAbsolutePath().toString());
    char[][] paddedGrid = AoCInputFileReader.addPaddingToGrid(grid);
    long solutionPartOne = Solution.partOne(paddedGrid);
    System.out.println("Part One: " + solutionPartOne); // 556367

    long solutionPartTwo = Solution.partTwo(paddedGrid);
    System.out.println("Part Two: " + solutionPartTwo); // 88373022 89471771
  }
}
