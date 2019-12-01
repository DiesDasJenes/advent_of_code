import java.util.ArrayList;
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

    public String getBoxName(String path) {
        List<List<Character>> puzzleInput = FileReader.readFileFToListOfCharListFrom(path);
        String output = "";
        for (List<Character> list: puzzleInput
             ) {
            List<Character> searchedValue = getSearchedList(list,puzzleInput);
            if(!searchedValue.isEmpty()){
                output = reduceDifference(list,searchedValue);
            }
        }
        return output;
    }

    private String reduceDifference(List<Character> list, List<Character> searchedValue) {
        StringBuilder stringBuilder = new StringBuilder();
        for (int i = 0; i < list.size(); i++){
            if(list.get(i).equals(searchedValue.get(i))){
                stringBuilder.append(searchedValue.get(i));
            }
        }
        return stringBuilder.toString();
    }

    public List<Character> getSearchedList(List<Character> currentList, List<List<Character>> puzzleInput){
        int difference=0;
        List<Character> searchedValue = new ArrayList<>();
        for (List<Character> list: puzzleInput
        ) {
            for (int i = 0; i < list.size(); i++){
                if(!list.get(i).equals(currentList.get(i))){
                    difference+=1;
                }
            }
            if(difference==1){
                return list;
            }
            difference=0;
        }

        return searchedValue;
    }
}
