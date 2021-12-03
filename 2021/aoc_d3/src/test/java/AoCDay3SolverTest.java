import org.junit.jupiter.api.Test;

import java.io.IOException;
import java.net.URISyntaxException;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;
import static org.assertj.core.api.Assertions.assertThatCode;

class AoCDay3SolverTest {

   List<String> TEST_INPUT = List.of(
            "00100",
            "11110",
            "10110",
            "10111",
            "10101",
            "01111",
            "00111",
            "11100",
            "10000",
            "11001",
            "00010",
            "01010");


  @Test
  void shouldReturnSumOfEachColumns() {
      var sumOfEachColumn = AoCDay3Solver.getSumOfEachColumn(TEST_INPUT);
      var expectedSumArray = new int[]{7, 5, 8, 7, 5};
      assertThat(sumOfEachColumn).isEqualTo(expectedSumArray);
  }

  @Test
  void shouldCalculateGammaRate() {
      var sumArray = new int[]{7, 5, 8, 7, 5};
      var gammaRate = AoCDay3Solver.getGammaRate(sumArray, TEST_INPUT.size());
      assertThat(gammaRate).isEqualTo("10110");
  }

  @Test
  void shouldInvertGammaRateToGetEpsilon(){
      var gammaRate = "10110";
      var expectedEpsilonRate = "01001";
      var actualEpsilonRate = AoCDay3Solver.getEpsilonRate(gammaRate);
      assertThat(actualEpsilonRate).isEqualTo(expectedEpsilonRate);
  }

  @Test
  void shouldSolvePartOne() {
      assertThatCode(() -> {
         var result=  AoCDay3Solver.solvePartOne(TEST_INPUT);
         assertThat(result).isEqualTo(198);
      }).doesNotThrowAnyException();
  }

    @Test
    void shouldSolvePartTwo() {
        assertThatCode(() -> {
            var result=  AoCDay3Solver.solvePartTwo(TEST_INPUT);
            assertThat(result).isEqualTo(230);
        }).doesNotThrowAnyException();
    }
}