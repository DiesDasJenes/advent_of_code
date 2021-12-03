use adc_d2::{get_solution_for_part_one, get_solution_for_part_two};

pub fn main() {
    println!("Hello, Wichtel!");

    let input_lines = adc_d2::util::get_input_lines_without_buffer();
    let solution_part_one = get_solution_for_part_one(&input_lines);
    println!("Part 1");
    println!("Result: {}", solution_part_one);

    let solution_part_two = get_solution_for_part_two(&input_lines);
    println!("Part 2");
    println!("Result: {}", solution_part_two);
}
