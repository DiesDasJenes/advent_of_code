package org.aoc;

import java.io.File;
import java.io.FileNotFoundException;
import java.io.FileReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Scanner;
import java.util.stream.Collectors;

public class AoCInputFileReader {

  private static List<String> readFileToListOfStringsFrom(String path) {
    Scanner scanner = new Scanner(path);
    List<String> list = new ArrayList<>();
    while (scanner.hasNextInt()) {
      list.add(scanner.next());
    }
    return list;
  }

  private static List<List<Character>> readFileFToListOfCharListFrom(String path) {
    List<List<Character>> list = new ArrayList<>();

    try {
      Scanner scanner = new Scanner(new File(path));

      while (scanner.hasNext()) {
        List<Character> sublist = new ArrayList<>();
        scanner.next().chars().forEach(value -> sublist.add((char) value));

        list.add(sublist);
      }
    } catch (FileNotFoundException e) {
      e.printStackTrace();
    }
    return list;
  }

  private static char[][] readFileFToTwoDimensionalArrayFrom(String path) {
    List<String> lines = new ArrayList<>();

    try {
      FileReader fileReader = new FileReader(path);

      Scanner scanner = new Scanner(fileReader);

      while (scanner.hasNextLine()) {
        lines.add(scanner.nextLine());
      }

      scanner.close();
    } catch (FileNotFoundException e) {
      throw new RuntimeException(e);
    }
    int rows = lines.size();
    int cols = rows > 0 ? lines.getFirst().length() : 0;
    char[][] grid = new char[rows][cols];

    for (int i = 0; i < rows; i++) {
      for (int j = 0; j < cols; j++) {
        grid[i][j] = lines.get(i).charAt(j);
      }
    }

    return grid;
  }

  public static char[][] addPaddingToGrid(char[][] grid) {
    char[][] paddedGrid = new char[grid.length][grid[0].length+2];
    for (int x = 0; x < grid[0].length; x++){
        List<Character> temp = new ArrayList<>();
        temp.add('.');
        for (char c : grid[x]){
          temp.add(c);
        }
        temp.add('.');
        paddedGrid[x] = temp.stream().map(Object::toString).collect(Collectors.joining()).toCharArray();
    }

    return paddedGrid;
  }

  public static List<List<Integer>> readFileToListOfIntegerArrays(String path) {
    List<List<Integer>> list  = createInitialListWithIntegerArrays(path);
    try {
      FileReader fileReader = new FileReader(path);

      Scanner scanner = new Scanner(fileReader);
      while (scanner.hasNextLine()) {
        String line = scanner.nextLine();
        String[] columns = line.trim().split("\\s+");
        for(int i = 0; i < columns.length; i++) {
          list.get(i).add(Integer.valueOf(columns[i]));
        }
      }

      scanner.close();
    } catch (FileNotFoundException e) {
      throw new RuntimeException(e);
    }
    return list;
  }

  private static List<List<Integer>> createInitialListWithIntegerArrays(String path) {
    ArrayList<List<Integer>> list  = new ArrayList<>();
    try {
      FileReader fileReader = new FileReader(path);

      Scanner scanner = new Scanner(fileReader);
      if (scanner.hasNextLine()) {
        String firstLine = scanner.nextLine();
        String[] columns = firstLine.trim().split("\\s+");
        int numColumns = columns.length;

        for (int i = 0; i < numColumns; i++) {
          list.add(new ArrayList<>());
        }
      }
      scanner.close();
    } catch (FileNotFoundException e) {
      throw new RuntimeException(e);
    }
    return list;
  }

  @SuppressWarnings("unchecked")
  public static <T> T readPuzzleInput(String path, String returnType) {
      return switch (returnType) {
          case "char[][]" -> (T) readFileFToTwoDimensionalArrayFrom(path);
          case "List<List<Character>>" -> (T) readFileFToListOfCharListFrom(path);
          case "List<String>" -> (T) readFileToListOfStringsFrom(path);
          case "List<List<Integer[]>" -> (T) readFileToListOfIntegerArrays(path);
          default -> throw new IllegalArgumentException("Unsupported return type: " + returnType);
      };
  }
}
