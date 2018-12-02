import org.junit.Before;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.CoreMatchers.notNullValue;
import static org.junit.Assert.*;
import static org.mockito.ArgumentMatchers.eq;
import static org.mockito.Mockito.verify;

// https://adventofcode.com/2018/day/1

public class ChronalCalibrationTest {

    private ChronalCalibration chronalCalibration;

    @Before
    public void setUp() throws Exception {
        chronalCalibration = new ChronalCalibration();
    }

    @Test
    public void shouldSUMAllValues() {
        int actualFrequency = chronalCalibration.getLastFrequency("src\\test\\resources\\testinput");

        assertThat(actualFrequency, is(3));
    }


    @Test
    public void shouldReturnLastFrequency() {
        int actualFrequency = chronalCalibration.getLastFrequency("src\\test\\resources\\input");

        System.out.println("Part One:" +actualFrequency);
    }

    @Test
    public void shouldReturnFirstRepeatedFrequency() {
        int repeatedFrequency = chronalCalibration.getFirstRepeatedFrequency("src\\test\\resources\\input");

        System.out.println("Part Two:" +repeatedFrequency);
    }

    @Test
    public void shouldReturnFirstRepeatedFrequency1() {
        int repeatedFrequency = chronalCalibration.getFirstRepeatedFrequency("src\\test\\resources\\testinput");

        assertThat(repeatedFrequency, is(2));
    }

    @Test
    public void shouldReturnFirstRepeatedFrequency2() {
        int repeatedFrequency = chronalCalibration.getFirstRepeatedFrequency("src\\test\\resources\\testinput2");

        assertThat(repeatedFrequency, is(1));
    }

    @Test
    public void shouldReturnFirstRepeatedFrequency3() {
        int repeatedFrequency = chronalCalibration.getFirstRepeatedFrequency("src\\test\\resources\\testinput3");

        assertThat(repeatedFrequency, is(10));
    }

    @Test
    public void shouldReturnFirstRepeatedFrequency4() {
        int repeatedFrequency = chronalCalibration.getFirstRepeatedFrequency("src\\test\\resources\\testinput4");

        assertThat(repeatedFrequency, is(14));
    }

    @Test
    public void shouldReturnFirstRepeatedFrequency5() {
        int repeatedFrequency = chronalCalibration.getFirstRepeatedFrequency("src\\test\\resources\\testinput5");

        assertThat(repeatedFrequency, is(5));
    }
}
