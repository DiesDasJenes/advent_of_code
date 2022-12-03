mod util;

fn parse_puzzle_input(file_path: &str) -> (String, Vec<String>) {
    let puzzle_input: Vec<String> = util::get_input_lines_without_buffer(file_path);
    let puzzle_line = puzzle_input.first().unwrap().clone();
    let polymer_template = puzzle_input.into_iter().skip(2).collect::<Vec<String>>();

    (puzzle_line, polymer_template)
}

fn parse_polymer_template_line(line: &String) -> (String, String) {
    let mut split = line.split(" -> ");
    let search_param = split.next().unwrap().clone().to_string();
    let mut replacement_string = search_param.clone();
    replacement_string.insert(1, split.next().unwrap().chars().next().unwrap());

    (search_param, replacement_string)
}

fn get_polymer(mut polymer_template: String, pair_insertion_rules: Vec<String>) -> String {
    for pair_insertion_rule in pair_insertion_rules {
        let (search_param, replacement_string) = parse_polymer_template_line(&pair_insertion_rule);
        polymer_template = polymer_template.replace(&search_param, &*replacement_string);
        println!("Line: {} and its template: {}", polymer_template, pair_insertion_rule)
    }
    polymer_template
}

#[cfg(test)]
mod test {
    use crate::{get_polymer, parse_polymer_template_line, parse_puzzle_input};

    #[test]
    fn should_return_tuple_with_puzzle_line_and_polymer_template() {
        let (polymer_template, pair_insertion_rules) =
            parse_puzzle_input("./resources/puzzle_test_input.txt");

        assert_eq!(polymer_template, "NNCB");
        assert_eq!(pair_insertion_rules.first().unwrap(), "CH -> B");
        assert_eq!(pair_insertion_rules.len(), 16);
    }

    #[test]
    fn should_return_tuple_with_search_param_and_replacement_string() {
        let (_, pair_insertion_rules) =
            parse_puzzle_input("./resources/puzzle_test_input.txt");

        let (search_param, replacement_string) =
            parse_polymer_template_line(pair_insertion_rules.first().unwrap());
        assert_eq!(search_param, "CH");
        assert_eq!(replacement_string, "CBH");
    }

    #[test]
    fn should_return_solve_part_one() {
        let (polymer_template, pair_insertion_rules) =
            parse_puzzle_input("./resources/puzzle_test_input.txt");

        let polymer = get_polymer(polymer_template, pair_insertion_rules);
        assert_eq!(polymer, "NCNBCHB");
    }
}
