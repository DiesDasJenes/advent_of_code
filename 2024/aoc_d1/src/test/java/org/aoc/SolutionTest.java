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

  private static Stream<Arguments> provideTestCases() {
    return Stream.of(
        of(List.of(List.of(1), List.of(1)), 0L),
        of(List.of(List.of(1), List.of(3)), 2L),
        of(List.of(List.of(1, 1, 2), List.of(1, 2, 2)), 1L),
        of(List.of(List.of(3), List.of(1)), 2L));
  }

  @ParameterizedTest
  @MethodSource("provideTestCases")
  void shouldFindDistanceBetweenNumbers(List<List<Integer>> someList, long expected) {
    long solutionToPartOne = Solution.partOne(someList);

    assertThat(solutionToPartOne).isEqualTo(expected);
  }
}
