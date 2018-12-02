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
        chronalCalibration = new ChronalCalibration(0);
    }

    @Test
    public void shouldSUMAllValues() {
        chronalCalibration.getLastFrequency("src\\test\\resources\\testinput");

        assertThat(chronalCalibration.frequency, is(3));
    }
    
    @Test
    public void shouldReturnLastFrequency() {
        chronalCalibration.getLastFrequency("src\\test\\resources\\input");

        System.out.println(chronalCalibration.frequency);
    }
}