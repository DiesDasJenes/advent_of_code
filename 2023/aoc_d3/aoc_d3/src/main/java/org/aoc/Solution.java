package org.aoc;

import java.util.ArrayList;
import java.util.List;

public class Solution {

  private static boolean checkForSymbols(int x, int y, char[][] input) {
    int amountOfCols = input[x].length;
    int amountOfRows = input.length;

    Direction[] directions = Direction.values();

    for (Direction direction : directions) {
      switch (direction) {
        case RIGHT:
          if (y < amountOfCols - 1 && isSymbol(input[x][y + 1])) {
            return true;
          }
          break;
        case LEFT:
          if (y > 0 && isSymbol(input[x][y - 1])) {
            return true;
          }
          break;
        case ABOVE:
          if (x > 0 && isSymbol(input[x - 1][y])) {
            return true;
          }
          break;
        case BELOW:
          if (x < amountOfRows - 1 && isSymbol(input[x + 1][y])) {
            return true;
          }
          break;
        case UPPER_RIGHT:
          if (x > 0 && y < amountOfCols - 1 && isSymbol(input[x - 1][y + 1])) {
            return true;
          }
          break;
        case UPPER_LEFT:
          if (x > 0 && y > 0 && isSymbol(input[x - 1][y - 1])) {
            return true;
          }
          break;
        case LOWER_RIGHT:
          if (x < amountOfRows - 1 && y < amountOfCols - 1 && isSymbol(input[x + 1][y + 1])) {
            return true;
          }
          break;
        case LOWER_LEFT:
          if (x < amountOfRows - 1 && y > 0 && isSymbol(input[x + 1][y - 1])) {
            return true;
          }
          break;
      }
    }
    return false;
  }

  private enum Direction {
    RIGHT,
    LEFT,
    ABOVE,
    BELOW,
    UPPER_RIGHT,
    UPPER_LEFT,
    LOWER_RIGHT,
    LOWER_LEFT
  }

  private static boolean isSymbol(char c) {
    return !(Character.isLetterOrDigit(c) || c == '.');
  }

  public static long partOne(char[][] input) {
    StringBuilder builder = new StringBuilder();
    long sum = 0L;
    boolean isValidNumberWithSymbol = false;
    for (int i = 0; i < input.length; i++) {
      for (int j = 0; j < input[0].length; j++) {
        char current = input[i][j];
        if (Character.isDigit(current)) {
          builder.append(current);
          isValidNumberWithSymbol = isValidNumberWithSymbol || checkForSymbols(i, j, input);
        } else {
          if (!builder.isEmpty() && isValidNumberWithSymbol) {
            sum += Integer.parseInt(builder.toString());
            isValidNumberWithSymbol = false;
          }
          builder.setLength(0);
        }
      }
    }
    return sum;
  }

  public static long partTwo(char[][] input) {
    long sum = 0L;
    for (int i = 0; i < input.length; i++) {
      for (int j = 0; j < input[0].length; j++) {
        char current = input[i][j];
        if (current == '*') {
          List<Integer> digits = collectAllDigits(i, j, input);
          System.out.println(digits);
          if (digits.size() == 2) {
            Integer gearRatio =
                digits.stream().reduce((integer, integer2) -> integer * integer2).get();
            sum += gearRatio;
          }
        }
      }
    }
    return sum;
  }

  private static Integer getNumber(char[][] input, int pointX, int pointY) {
    StringBuilder stringBuilder = new StringBuilder();
    while (pointY > 0 && Character.isDigit(input[pointX][pointY])) {
      pointY--;
    }
    while (pointY < input[pointX].length && Character.isDigit(input[pointX][pointY + 1])) {
      stringBuilder.append(input[pointX][pointY + 1]);
      pointY++;
    }
    return Integer.valueOf(stringBuilder.toString());
  }

  private static List<Integer> collectAllDigits(int startPointX, int startPointY, char[][] input) {
    List<Integer> digits = new ArrayList<>();
    int amountOfCols = input[startPointX].length;
    int amountOfRows = input.length;

    Direction[] directions = Direction.values();

    for (Direction direction : directions) {
      switch (direction) {
        case RIGHT:
          if (startPointY < amountOfCols - 1
              && Character.isDigit(input[startPointX][startPointY + 1])) {
            Integer number = getNumber(input, startPointX, startPointY + 1);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
        case LEFT:
          if (startPointY > 0 && Character.isDigit(input[startPointX][startPointY - 1])) {
            Integer number = getNumber(input, startPointX, startPointY - 1);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
        case ABOVE:
          if (startPointX > 0 && Character.isDigit(input[startPointX - 1][startPointY])) {
            Integer number = getNumber(input, startPointX - 1, startPointY);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
        case BELOW:
          if (startPointX < amountOfRows - 1
              && Character.isDigit(input[startPointX + 1][startPointY])) {
            Integer number = getNumber(input, startPointX + 1, startPointY);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
        case UPPER_RIGHT:
          if (startPointX > 0
              && startPointY < amountOfCols - 1
              && Character.isDigit(input[startPointX - 1][startPointY + 1])) {
            Integer number = getNumber(input, startPointX - 1, startPointY + 1);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
        case UPPER_LEFT:
          if (startPointX > 0
              && startPointY > 0
              && Character.isDigit(input[startPointX - 1][startPointY - 1])) {
            Integer number = getNumber(input, startPointX - 1, startPointY - 1);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
        case LOWER_RIGHT:
          if (startPointX < amountOfRows - 1
              && startPointY < amountOfCols - 1
              && Character.isDigit(input[startPointX + 1][startPointY + 1])) {
            Integer number = getNumber(input, startPointX + 1, startPointY + 1);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
        case LOWER_LEFT:
          if (startPointX < amountOfRows - 1
              && startPointY > 0
              && Character.isDigit(input[startPointX + 1][startPointY - 1])) {
            Integer number = getNumber(input, startPointX + 1, startPointY - 1);
            if (!digits.contains(number)) {
              digits.add(number);
            }
          }
      }
    }
    return digits;
  }
}
