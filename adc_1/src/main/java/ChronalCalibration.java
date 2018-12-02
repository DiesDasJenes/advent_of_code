import java.io.File;
import java.io.IOException;
import java.util.ArrayList;
import java.util.List;
import java.util.Scanner;

public class ChronalCalibration {

    public ChronalCalibration() {
    }

    public int getLastFrequency(String filePath) {
        List<Integer> frequencies = readInputFile(filePath);
        return frequencies.stream().mapToInt(integer -> integer).sum();
    }

    public Integer getFirstRepeatedFrequency(String filePath) {
        List<Integer> frequencies = readInputFile(filePath);
        int resultingFrequency = 0;
        ArrayList<Integer> stepsInBetween = new ArrayList<>();

        while (true) {
            for (int integer : frequencies
            ) {
                int tmp = stepsInBetween.size() == 0 ? 0 : stepsInBetween.get(stepsInBetween.size()-1);
                resultingFrequency = tmp + integer;
                if(stepsInBetween.contains(resultingFrequency)){
                    return resultingFrequency;
                }
                stepsInBetween.add(resultingFrequency);
            }
        }
    }


    private List<Integer> readInputFile(String filePath) {
        List<Integer> metrics = new ArrayList<>();

        try {
            Scanner scanner = new Scanner(new File(filePath));
            while (scanner.hasNextInt()) {
                metrics.add(scanner.nextInt());
            }
        } catch (IOException e) {
            throw new RuntimeException("Could not find the file", e);
        }
        return metrics;
    }
}
