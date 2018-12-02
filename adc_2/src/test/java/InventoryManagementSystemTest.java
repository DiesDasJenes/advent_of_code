import com.google.common.collect.ImmutableBiMap;
import org.junit.Before;
import org.junit.Test;

import java.util.Arrays;
import java.util.Map;

import static org.hamcrest.CoreMatchers.is;
import static org.hamcrest.CoreMatchers.notNullValue;
import static org.junit.Assert.*;
import static org.mockito.ArgumentMatchers.any;
import static org.mockito.Mockito.mock;
import static org.mockito.Mockito.when;

public class InventoryManagementSystemTest {

    private InventoryManagementSystem managementSystem;
    private final String puzzleInput = "src\\test\\resources\\input";
    private final String puzzleTestInput = "src\\test\\resources\\testinput";
    @Before
    public void setUp() throws Exception {
        managementSystem = new InventoryManagementSystem();
    }

    @Test
    public void shouldReturnListOfRepeatedLetters() {
        assertThat(managementSystem.getRepeatedLettersCount(puzzleTestInput), is(notNullValue()));
    }

    @Test
    public void shouldCalculateChecksum() {
        Map<Integer, Integer> expectedMap = ImmutableBiMap.of(2, 4,3,3);

        assertThat(managementSystem.calculateChecksum(expectedMap),is(12));
    }

    @Test
    public void shouldReturnValidValueForTestInput() {
        Map<Integer, Integer> expectedMap = managementSystem.getRepeatedLettersCount(puzzleTestInput);

        assertThat(managementSystem.calculateChecksum(expectedMap),is(12));
    }

    @Test
    public void shouldReturnValidValueForPartOne() {
        Map<Integer, Integer> expectedMap = managementSystem.getRepeatedLettersCount(puzzleInput);

        assertThat(managementSystem.calculateChecksum(expectedMap),is(6150));
    }

    @Test
    public void shouldReturnTrueAsAIsNotRepeated() {
        assertThat(managementSystem.isRepeated(2,'a', Arrays.asList('a','b','c','d','e','f')),is(false));
    }

    @Test
    public void shouldReturnTrueAsAIsRepeated() {
        assertThat(managementSystem.isRepeated(2,'a', Arrays.asList('b','a','b','a','b','c')),is(true));
    }
}