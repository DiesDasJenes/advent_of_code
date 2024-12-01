package org.aoc;

import org.junit.jupiter.api.Test;

import java.net.URISyntaxException;
import java.nio.file.Paths;

import static org.assertj.core.api.Assertions.assertThat;

class AoCInputFileReaderTest {

  @Test
  void shouldReadInInputAsGrid() throws URISyntaxException {
    char[][] grid =
        AoCInputFileReader.readFileFToTwoDimensionalArrayFrom(
            Paths.get("src/test/resources/example-puzzle-input.txt").toAbsolutePath().toString());

    assertThat(grid.length).isEqualTo(10);
  }

  @Test
  void shouldAddDotPaddingToGrid() {
    char[][] grid = {{'1', '1'}, {'1', '1'}};
    char[][] expectedGrid = {{'.', '1', '1', '.'}, {'.', '1', '1', '.'}};

    assertThat(AoCInputFileReader.addPaddingToGrid(grid)).isEqualTo(expectedGrid);
  }
}
