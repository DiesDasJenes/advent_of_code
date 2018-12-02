import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class InventoryManagementSystem {

    public Map<Integer, Integer> getRepeatedLettersCount(String path) {
        List<List<Character>> puzzleInput = FileReader.readFileFToListOfCharListFrom(path);
        Map<Integer, Integer> countOfLetters = new HashMap<>();
        int two = 0;
        int three = 0;
        for (List<Character> letters : puzzleInput
        ) {
            two += hasSubListRepeated(2, letters) ? 1 : 0;
            three += hasSubListRepeated(3, letters) ? 1 : 0;
        }
        countOfLetters.put(2, two);
        countOfLetters.put(3, three);
        return countOfLetters;
    }

    public boolean hasSubListRepeated(int iteration, List<Character> letters) {
        for (Character c : letters
        ) {
            if (isRepeated(iteration, c, letters)) {
                return true;
            }
        }
        return false;
    }

    public boolean isRepeated(int iterations, char letter, List<Character> letters) {
        int internalCount = letters.stream().mapToInt(l -> l.equals(letter) ? 1 : 0).sum();
        return internalCount == iterations;
    }

    public int calculateChecksum(Map<Integer, Integer> repeatedLetters) {
        return repeatedLetters.get(2) * repeatedLetters.get(3);
    }
}
