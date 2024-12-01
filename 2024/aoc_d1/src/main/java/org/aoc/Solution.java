package org.aoc;

import java.util.ArrayList;
import java.util.List;

public class Solution {

  public static long partOne(List<List<Integer>> inputs) {
    List<List<Integer>> sortedInputs = new ArrayList<>();
    long distance = 0L;
    for (List<Integer> input : inputs){
      sortedInputs.add(input.stream().sorted().toList());
    }
    for(int i = 0; i < sortedInputs.getFirst().size(); i++) {
      distance += Math.abs((long) sortedInputs.getLast().get(i) - (long) sortedInputs.getFirst().get(i));
    }
    return distance;
  }

  public static long partTwo() {
    return 0;
  }
}
