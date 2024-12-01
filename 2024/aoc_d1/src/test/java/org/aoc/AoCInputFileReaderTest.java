package org.aoc;

import org.junit.jupiter.api.Test;

import java.net.URISyntaxException;
import java.nio.file.Paths;
import java.util.List;

import static org.assertj.core.api.Assertions.assertThat;

class AoCInputFileReaderTest {

  @Test
  void shouldReadInInputAsGrid() throws URISyntaxException {
    char[][] grid =
        AoCInputFileReader.readPuzzleInput(
            Paths.get("src/test/resources/example-grid-puzzle-input.txt").toAbsolutePath().toString(), "char[][]");

    assertThat(grid.length).isEqualTo(10);
  }

  @Test
  void shouldAddDotPaddingToGrid() {
    char[][] grid = {{'1', '1'}, {'1', '1'}};
    char[][] expectedGrid = {{'.', '1', '1', '.'}, {'.', '1', '1', '.'}};

    assertThat(AoCInputFileReader.addPaddingToGrid(grid)).isEqualTo(expectedGrid);
  }

  @Test
  void shouldReadInInputAsListOfIntegerArrays() {
    List<List<Integer>> input =
            AoCInputFileReader.readFileToListOfIntegerArrays(
                    Paths.get("src/test/resources/example-list-integer-arrays-puzzle-input.txt").toAbsolutePath().toString());

    assertThat(input.size()).isEqualTo(2);
    assertThat(input.get(0).size()).isEqualTo(6);
    assertThat(input.get(1).size()).isEqualTo(6);
  }
}
