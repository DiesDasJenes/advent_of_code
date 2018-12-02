import java.io.File;
import java.io.IOException;
import java.util.*;

public class ChronalCalibration {

    public int frequency;


    public ChronalCalibration(int frequency) {
        this.frequency = frequency;
    }

    public void getLastFrequency(String filePath) {
        List<Integer> metric = readInputFile(filePath);
        metric.forEach(integer -> frequency+=integer);
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
