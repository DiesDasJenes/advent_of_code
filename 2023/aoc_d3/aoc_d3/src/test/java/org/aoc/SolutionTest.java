package org.aoc;

import org.junit.jupiter.api.Test;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.Arguments;
import org.junit.jupiter.params.provider.MethodSource;

import java.util.stream.Stream;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.in;
import static org.junit.jupiter.params.provider.Arguments.of;

class SolutionTest {

  private static Stream<Arguments> provideTestCases() {
    return Stream.of(
        of(new char[][] {{'4', '6', '7', '*', '.', '1', '1', '4'}}, 467),
        of(new char[][] {{'*', '2', '3', '1', '.', '4', '5', '6', '.', '.'}}, 231),
        of(new char[][] {{'.', '1', '1', '1', '*', '2', '2', '2', '.'}}, 333),
        of(new char[][] {{'.', '*', '1', '1', '1', '*', '.', '.', '.'}}, 111),
        of(
            new char[][] {
              {
                '#', '.', '.', '.', '.', '.', '.', '.',
              },
              {
                '7', '8', '9', '.', '.', '5', '3', '2',
              }
            },
            789),
        of(
            new char[][] {
              {
                '.', '.', '.', '.', '.', '.', '.', '.',
              },
              {
                '6', '6', '6', '.', '.', '4', '3', '4',
              },
              {
                '.', '.', '.', '.', '.', '.', '#', '.',
              }
            },
            434),
        of(
            new char[][] {
              {
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.',
              },
              {
                '.', '6', '6', '6', '.', '.', '4', '3', '4', '.', '.',
              },
              {
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
              }
            },
            434),
        of(
            new char[][] {
              {
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
              },
              {
                '.', '6', '6', '6', '.', '.', '4', '3', '4', '.', '.',
              },
              {
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '#', '.',
              }
            },
            434),
        of(
            new char[][] {
              {
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
              },
              {
                '.', '6', '6', '6', '.', '.', '4', '3', '4', '.', '.',
              },
              {
                '.', '.', '.', '.', '.', '/', '.', '.', '.', '.', '.',
              }
            },
            434),
        of(
            new char[][] {
              {
                '.', '.', '.', '.', '.', '-', '.', '.', '.', '.', '.',
              },
              {
                '.', '6', '6', '6', '.', '.', '4', '3', '4', '.', '.',
              },
              {
                '.', '.', '.', '.', '.', '.', '.', '.', '.', '.', '.',
              }
            },
            434),
        of(
            new char[][] {
              {'4', '6', '7', '.', '.', '1', '1', '4', '.', '.'},
              {'.', '.', '.', '*', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '3', '5', '.', '.', '6', '3', '3', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.', '.'},
              {'6', '1', '7', '*', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '+', '.', '5', '8', '.'},
              {'.', '.', '5', '9', '2', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '7', '5', '5', '.'},
              {'.', '.', '.', '$', '.', '*', '.', '.', '.', '.'},
              {'.', '6', '6', '4', '.', '5', '9', '8', '.', '.'}
            },
            4361),
        of(
            new char[][] {
              {
                '.', '$', '.', '.', '.', '.', '.', '.',
              },
              {
                '6', '6', '6', '.', '.', '.', '.', '.',
              },
              {
                '.', '#', '.', '.', '.', '.', '#', '.',
              }
            },
            666),
        // Scenario: A number adjacent to multiple symbols
        of(
            new char[][] {
              {'1', '2', '3', '*', '.', '4', '5', '6', '.', '.'},
              {'*', '.', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.', '.'}
            },
            123 // '123' is surrounded by '*' symbols
            ),
        // Scenario: Symbols are adjacent but no numbers nearby
        of(
            new char[][] {
              {'*', '*', '.', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.'}
            },
            0 // No numbers adjacent to symbols
            ),
        // Scenario: Numbers in multiple rows with scattered symbols
        of(
            new char[][] {
              {'7', '8', '9', '#', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '#', '1', '0', '0', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.'}
            },
            889 // '789' has '#' nearby
            ),
        // Scenario: Numbers are adjacent but not to symbols
        of(
            new char[][] {
              {'5', '5', '5', '.', '.', '.', '.', '.', '.'},
              {'*', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'5', '5', '5', '.', '.', '.', '.', '.', '.'}
            },
            1110),
        // Scenario: Numbers are adjacent but not to symbols
        of(
            new char[][] {
              {'5', '5', '5', '.', '.', '.', '.', '.', '.'},
              {'*', '*', '.', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '.', '.', '.'}
            },
            555),
        // Scenario: Mixed grid with multiple symbols and numbers
        of(
            new char[][] {
              {'4', '0', '*', '5', '1', '+', '2', '6', '*', '.'},
              {'.', '.', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'3', '0', '.', '.', '#', '8', '9', '.', '.', '.'}
            },
            206),
        // Scenario: Mixed grid with multiple symbols and numbers
        of(
            new char[][] {
              {'4', '0', '*', '5', '1', '+', '2', '6', '*', '.'},
              {'3', '.', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'3', '0', '.', '.', '#', '8', '9', '.', '.', '.'}
            },
            206),
        of(new char[][] {{'4', '6', '7', '*', '.', '1', '1', '4'}}, 467),

        // Numbers adjacent to symbols in the middle
        of(new char[][] {{'*', '2', '3', '1', '.', '4', '5', '6', '.', '.'}}, 231),

        // Numbers completely surrounded by symbols
        of(new char[][] {{'.', '1', '1', '1', '*', '2', '2', '2', '.'}}, 333),

        // Edge case: Numbers partially surrounded by symbols
        of(new char[][] {{'.', '*', '1', '1', '1', '*', '.', '.', '.'}}, 111),

        // Multi-row grid with a mix of symbols and numbers
        of(
            new char[][] {
              {'.', '.', '.', '.', '.', '.', '.', '.'},
              {'7', '8', '9', '.', '.', '5', '3', '2'},
              {'.', '.', '.', '.', '.', '.', '#', '.'}
            },
            532),

        // Complex grid with numbers scattered and symbols
        of(
            new char[][] {
              {'4', '6', '7', '.', '.', '1', '1', '4', '.', '.'},
              {'.', '.', '.', '*', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '3', '5', '.', '.', '6', '3', '3', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.', '.'},
              {'6', '1', '7', '*', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '+', '.', '5', '8', '.'},
              {'.', '.', '5', '9', '2', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '7', '5', '5', '.'},
              {'.', '.', '.', '$', '.', '*', '.', '.', '.', '.'},
              {'.', '6', '6', '4', '.', '5', '9', '8', '.', '.'}
            },
            4361),

        // Simple scenario with numbers and symbols on the edges
        of(
            new char[][] {
              {'.', '$', '.', '.', '.', '.', '.', '.'},
              {'6', '6', '6', '.', '.', '.', '.', '.'},
              {'.', '#', '.', '.', '.', '.', '#', '.'}
            },
            666),

        // Numbers adjacent to multiple symbols
        of(
            new char[][] {
              {'1', '2', '3', '*', '.', '4', '5', '6', '.', '.'},
              {'*', '.', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.', '.'}
            },
            123),

        // Symbols without adjacent numbers
        of(
            new char[][] {
              {'*', '*', '.', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.'}
            },
            0),

        // Numbers surrounded by symbols in multiple rows
        of(
            new char[][] {
              {'7', '8', '9', '#', '.', '.', '.', '.', '.'},
              {'.', '.', '.', '#', '1', '0', '0', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '#', '.', '.'}
            },
            889),

        // Multiple numbers in the same row with different symbols
        of(
            new char[][] {
              {'4', '0', '*', '5', '1', '+', '2', '6', '*', '.'},
              {'.', '.', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'3', '0', '.', '.', '#', '8', '9', '.', '.', '.'}
            },
            206),

        // Single-digit numbers scattered
        of(
            new char[][] {
              {'5', '5', '5', '.', '.', '.', '.', '.', '.'},
              {'*', '.', '.', '.', '.', '.', '.', '.', '.'},
              {'5', '5', '5', '.', '.', '.', '.', '.', '.'}
            },
            1110),
        of(
            new char[][] {
              {'5', '5', '5', '.', '*', '.', '.', '.', '.'},
              {'.', '.', '.', '.', '.', '.', '.', '.', '*'},
              {'*', '.', '.', '.', '.', '.', '.', '.', '.'}
            },
            0),
        of(
            new char[][] {
              {'.', '.', '.', '.', '.', '.', '6', '4', '5'},
              {'1', '1', '5', '.', '.', '+', '.', '.', '.'},
              {'.', '.', '.', '*', '.', '.', '.', '.', '.'}
            },
            760));
  }

  @ParameterizedTest
  @MethodSource("provideTestCases")
  void shouldIdentifyAndSumNumbersWithSymbolsNearThem(char[][] grid, long expected) {
    long solutionToPartOne = Solution.partOne(grid);

    assertThat(solutionToPartOne).isEqualTo(expected);
  }

  @Test
  void shouldSplitNumberAtEndAndFront() {
    char[][] input = {
      {'.', '.', '.', '.', '.', '.', '6', '4', '5'},
      {'1', '1', '5', '.', '.', '+', '.', '.', '.'},
      {'.', '.', '.', '*', '.', '.', '.', '.', '.'}
    };

    assertThat(Solution.partOne(input)).isEqualTo(760);
  }

  @Test
  void shouldSplitNumberAtFrontAndEnd() {
    char[][] input = {
      {'6', '4', '5', '.', '.', '.', '.', '.', '.'},
      {'.', '.', '.', '+', '.', '+', '1', '1', '5'},
      {'.', '.', '.', '.', '.', '.', '.', '.', '.'}
    };

    assertThat(Solution.partOne(input)).isEqualTo(760);
  }

  @Test
  void shouldIdentifyGearsWithNumbersLeftAndRight() {
    char[][] input = {
      {'.', '6', '4', '5', '*', '1', '1', '2', '.'},
    };

    int expected = 72240;

    long actual = Solution.partTwo(input);

    assertThat(actual).isEqualTo(expected);
  }

  @Test
  void shouldIdentifyGearsWithNumbersAboveAndBelow() {
    char[][] input = {
      {'.', '6', '4', '5', '.', '.', '.', '.', '.'},
      {'.', '.', '.', '*', '.', '.', '.', '.', '.'},
      {'.', '.', '.', '1', '1', '2', '.', '.', '.'}
    };

    int expected = 72240;

    long actual = Solution.partTwo(input);

    assertThat(actual).isEqualTo(expected);
  }

    @Test
    void shouldIdentifyGearsWithNumbersAboveLeftAndAboveRight() {
        char[][] input = {
                {'.', '6', '4', '5', '.', '1', '1', '2', '.'},
                {'.', '.', '.', '.', '*', '.', '.', '.', '.'},
                {'.', '.', '.', '.', '.', '.', '.', '.', '.'}
        };

        int expected = 72240;

        long actual = Solution.partTwo(input);

        assertThat(actual).isEqualTo(expected);
    }

    @Test
    void shouldIdentifyGearsWithNumbersBelowLeftAndBelowRight() {
        char[][] input = {
                {'.', '.', '.', '.', '.', '.', '.', '.', '.'},
                {'.', '.', '.', '.', '*', '.', '.', '.', '.'},
                {'.', '6', '4', '5', '.', '1', '1', '2', '.'}
        };

        int expected = 72240;

        long actual = Solution.partTwo(input);

        assertThat(actual).isEqualTo(expected);
    }
}
