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

  public static List<String> readFileToListOfStringsFrom(String path) {
    Scanner scanner = new Scanner(path);
    List<String> list = new ArrayList<>();
    while (scanner.hasNextInt()) {
      list.add(scanner.next());
    }
    return list;
  }

  public static List<List<Character>> readFileFToListOfCharListFrom(String path) {
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

  public static char[][] readFileFToTwoDimensionalArrayFrom(String path) {
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
}
