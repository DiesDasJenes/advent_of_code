import java.io.File;
import java.io.FileNotFoundException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class FileReader {

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
}
