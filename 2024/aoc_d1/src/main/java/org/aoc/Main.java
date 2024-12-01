package org.aoc;

import java.nio.file.Paths;
import java.util.List;

public class Main {
  public static void main(String[] args) {
    List<List<Integer>> input =
        AoCInputFileReader.readFileToListOfIntegerArrays(
            Paths.get("src/main/resources/puzzle-input.txt").toAbsolutePath().toString());
    long solutionPartOne = Solution.partOne(input);
    System.out.println("Part One: " + solutionPartOne);

    long solutionPartTwo = Solution.partTwo(input);
    System.out.println("Part Two: " + solutionPartTwo);
  }
}
