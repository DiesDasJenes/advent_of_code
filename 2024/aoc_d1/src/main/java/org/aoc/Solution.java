package org.aoc;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Objects;

public class Solution {

  private static List<List<Integer>> getSortedInputs(List<List<Integer>> inputs) {
    List<List<Integer>> sortedInputs = new ArrayList<>();
    for (List<Integer> input : inputs){
      sortedInputs.add(input.stream().sorted().toList());
    }
    return sortedInputs;
  }

  public static long partOne(List<List<Integer>> inputs) {
    List<List<Integer>> sortedInputs = getSortedInputs(inputs);
    long distance = 0L;
    for(int i = 0; i < sortedInputs.getFirst().size(); i++) {
      distance += Math.abs((long) sortedInputs.getLast().get(i) - (long) sortedInputs.getFirst().get(i));
    }
    return distance;
  }

  public static long partTwo(List<List<Integer>> inputs) {
    List<List<Integer>> sortedInputs = getSortedInputs(inputs);
    HashMap<Integer, Long> occurences = new HashMap<>();
    long similarityScore = 0L;
    sortedInputs.get(0).forEach(current ->
            occurences.computeIfAbsent(current, key ->
                    sortedInputs.get(1).stream()
                            .filter(intRightColumn -> Objects.equals(key, intRightColumn))
                            .count()
            )
    );

    for (Integer current : sortedInputs.getFirst()){
      similarityScore += occurences.get(current) * current;
    }

    return similarityScore;
  }
}
