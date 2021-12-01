mod util;
use std::ops::Add;
use std::str::FromStr;

fn main() {
    if let Ok(lines) = util::read_lines("./puzzle_input.txt") {
        let numbers = lines
            .flat_map(|number| u32::from_str(number.unwrap().as_str()))
            .collect::<Vec<u32>>();
        let count = count_increases_of_measurements(&numbers);
        let list_of_sums = collect_sum_of_triple_measurements(&numbers);
        let count_of_sums = count_increases_of_measurements(&list_of_sums);
        println!("PART 1");
        println!("Count of increases: {}", count);

        println!("Part 2");
        println!("Count of increases of the sums: {}", count_of_sums);
    }
}

fn count_increases_of_measurements(measurements: &Vec<u32>) -> u32 {
    let mut count_of_increases: u32 = 0;
    let mut latest_increased_measurement: &u32 =
        measurements.first().unwrap();

    for measurement in measurements {
        if measurement > latest_increased_measurement {
            count_of_increases = count_of_increases.add(1)
        }
        latest_increased_measurement = measurement;
    }

    return count_of_increases;
}

fn collect_sum_of_triple_measurements(measurements: &Vec<u32>) -> Vec<u32> {
 let triples = measurements.windows(3);
 let mut collection_sums:Vec<u32> = Vec::new();
    for triple in triples {
        let sum = triple.iter().sum();
        collection_sums.push(sum);
    }

    return collection_sums
}



#[cfg(test)]
mod test {
    use crate::{collect_sum_of_triple_measurements, count_increases_of_measurements};

    #[test]
    fn should_increase_counter_when_measurements_increased() {
        let measurements_with_increases: Vec<u32> =
            Vec::from([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(
            count_increases_of_measurements(&measurements_with_increases),
            7
        )
    }

    #[test]
    fn should_not_increase_counter_when_measurements_decreased() {
        let measurements_with_decreases: Vec<u32> = Vec::from([3, 2, 1]);
        assert_eq!(
            count_increases_of_measurements(&measurements_with_decreases),
            0
        )
    }

    #[test]
    fn should_collect_sum_of_three_measurements() {
        let measurements: Vec<u32> =
            Vec::from([199, 200, 208, 210, 200, 207, 240, 269, 260, 263]);
        assert_eq!(
            collect_sum_of_triple_measurements(&measurements),
            Vec::from([607, 618, 618, 617, 647, 716, 769, 792])
        )
    }


}
