mod util;
use regex::Regex;

fn get_parsed_input(input: Vec<String>) -> Vec<String> {

    let regex = Regex::new(r"([0-9]+),([0-9]+) -> ([0-9]+),([0-9]+)$").unwrap();
    regex.captures_read_at()
    return Vec::new();
}


#[cfg(test)]
mod test {

    #[test]
    fn should_return_parsed_input () {

    }
}