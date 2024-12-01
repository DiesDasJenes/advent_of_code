package org.aoc;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.List;
import java.util.stream.Stream;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.in;
import static org.junit.jupiter.params.provider.Arguments.of;

class SolutionTest {

  private static Stream<Arguments> provideTestCasesP1() {
    return Stream.of(
            of(List.of(List.of(1), List.of(1)), 0L),
            of(List.of(List.of(1), List.of(3)), 2L),
            of(List.of(List.of(1, 1, 2), List.of(1, 2, 2)), 1L),
            of(List.of(List.of(3), List.of(1)), 2L));
  }

  private static Stream<Arguments> provideTestCasesP2() {
    return Stream.of(
            of(List.of(List.of(1), List.of(1)), 1L),
            of(List.of(List.of(1), List.of(3)), 0L),
            of(List.of(List.of(3,4, 2, 1,3,3), List.of(4, 3, 5,3,9,3)), 31L));
  }

  @ParameterizedTest
  @MethodSource("provideTestCasesP1")
  void shouldFindDistanceBetweenNumbers(List<List<Integer>> someList, long expected) {
    long solutionToPartOne = Solution.partOne(someList);

    assertThat(solutionToPartOne).isEqualTo(expected);
  }

  @ParameterizedTest
  @MethodSource("provideTestCasesP2")
  void shouldFindSimilarityScoreOfNumbers(List<List<Integer>> someList, long expected) {
    long solutionToPartOne = Solution.partTwo(someList);

    assertThat(solutionToPartOne).isEqualTo(expected);
  }
}
