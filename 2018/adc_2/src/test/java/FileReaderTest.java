import org.hamcrest.CoreMatchers;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.CoreMatchers.notNullValue;
import static org.junit.Assert.*;

public class FileReaderTest {

    private final String puzzleInput = "src\\test\\resources\\input";
    private final String puzzleTestInput = "src\\test\\resources\\testinput";

    @Test
    public void shouldReadFileToListOfStrings() {
        assertThat(FileReader.readFileToListOfStringsFrom(puzzleTestInput), is(notNullValue()));
    }

    @Test
    public void shouldReadFileFToListOfCharList() {
        assertThat(FileReader.readFileFToListOfCharListFrom(puzzleTestInput), is(notNullValue()));
        assertThat(FileReader.readFileFToListOfCharListFrom(puzzleTestInput).size(), is(7));
    }
}