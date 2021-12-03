import java.io.IOException;
import java.net.URISyntaxException;
import java.nio.file.Files;
import java.nio.file.Path;
import java.util.Arrays;
import java.util.List;
import java.util.Objects;
import java.util.function.BiPredicate;
import java.util.stream.Collectors;

public class AoCDay3Solver {
  public static void main(String[] args) throws URISyntaxException, IOException {
    var stringStream =
        Files.lines(
            Path.of(
                Objects.requireNonNull(AoCDay3Solver.class.getResource("puzzle_input.txt"))
                    .toURI()));
    var lines = stringStream.collect(Collectors.toList());
    System.out.println("Part 1 Puzzle solution: " + solvePartOne(lines));
    System.out.println("Part 2 Puzzle solution: " + solvePartTwo(lines));

  }

  static int[] getSumOfEachColumn(List<String> test_input) {
    var sumsArray = new int[test_input.get(0).length()];

    test_input.forEach(
        singleBinaryNumber -> {
          for (int i = 0; i < singleBinaryNumber.length(); i++) {
            sumsArray[i] += Integer.parseInt(singleBinaryNumber.substring(i, i + 1));
          }
        });
    return sumsArray;
  }

  public static String getGammaRate(int[] inputArray, long lengthOfInputArray) {

    return Arrays.stream(inputArray)
        .map(
            singleDigit -> {
              if (singleDigit >= lengthOfInputArray / 2) {
                return '1';
              } else {
                return '0';
              }
            })
        .collect(StringBuilder::new, StringBuilder::appendCodePoint, StringBuilder::append)
        .toString();
  }

  static String getEpsilonRate(String gammaRate) {
    return Arrays.stream(gammaRate.split(""))
        .map(chr -> (chr.equals("1")) ? "0" : "1")
        .collect(Collectors.joining(""));
  }

  static int getMostCommonDigitForColumnByIndex(List<String> inputArray, int index) {
    int sumOfIndexColumn =
        Integer.parseInt(inputArray.stream()
            .reduce("0",(line1, line2) -> line1 = String.valueOf(Integer.parseInt(line1) + Integer.parseInt(String.valueOf(line2.charAt(index))))));

    return (double) sumOfIndexColumn >= (double) inputArray.size() / 2 ? 1 : 0;
  }

  static String determineGasLevel(
      List<String> inputArray, int columnIndex, BiPredicate<Character, Integer> filterFunction) {
    if (inputArray.size() == 1) {
      return String.valueOf(inputArray);
    }

    var mostCommonDigit = getMostCommonDigitForColumnByIndex(inputArray, columnIndex);
    return determineGasLevel(
        inputArray.stream()
            .filter(
                singleBinaryString ->
                    filterFunction.test(singleBinaryString.charAt(columnIndex), mostCommonDigit))
            .collect(Collectors.toList()),
        columnIndex + 1,
        filterFunction);
  }

  static int determineOxygenGeneratorRating(List<String> inputArray) {
    BiPredicate<Character, Integer> filterFunction =
        (Character chr, Integer mostCommonDigit) ->
            Character.getNumericValue(chr) == mostCommonDigit;
      var determinedGasLevel = determineGasLevel(inputArray, 0, filterFunction);
      return Integer.parseInt(determinedGasLevel.replace("[","").replace("]",""), 2);
  }

    static int determineCO2ScrubberRating(List<String> inputArray) {
        BiPredicate<Character, Integer> filterFunction =
                (Character chr, Integer mostCommonDigit) ->
                        Character.getNumericValue(chr) != mostCommonDigit;
        var determinedGasLevel = determineGasLevel(inputArray, 0, filterFunction);
        return Integer.parseInt(determinedGasLevel.replace("[","").replace("]",""), 2);
    }

    static int solvePartOne(List<String> lines) {
        var sumOfEachColumn = getSumOfEachColumn(lines);
        var gammaRate = getGammaRate(sumOfEachColumn, lines.size());
        var epsilonRate = getEpsilonRate(gammaRate);
        return Integer.parseInt(gammaRate, 2) * Integer.parseInt(epsilonRate, 2);
    }

  static int solvePartTwo(List<String> lines) {
      var oxygenGeneratorRating = determineOxygenGeneratorRating(lines);
      var co2ScrubberRating = determineCO2ScrubberRating(lines);

      return oxygenGeneratorRating* co2ScrubberRating;
  }
}
