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
        of(List.of(1), 0));
  }

  @ParameterizedTest
  @MethodSource("provideTestCases")
  void shouldIdentifyAndSumNumbersWithSymbolsNearThem(List<Integer> someList, long expected) {
    long solutionToPartOne = Solution.partOne();

    assertThat(solutionToPartOne).isEqualTo(expected);
  }


}
