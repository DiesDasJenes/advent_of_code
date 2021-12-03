mod util;


fn get_gamma_and_epsilon(input: Vec<String>) -> (String,String) {
    let mut gamma = String::new();
    for line in input {

    }
    (gamma,"0".to_string())
}

#[cfg(test)]
mod test {
    use crate::get_gamma_and_epsilon;

    fn get_test_input() -> Vec<String> {
        Vec::from([
            "00100".to_string(), "11110".to_string(), "10110".to_string(), "10111".to_string(), "10101".to_string(), "01111".to_string(), "00111".to_string(), "11100".to_string(), "10000".to_string(),
            "11001".to_string(), "00010".to_string(), "01010".to_string(),
        ])
    }

    #[test]
    fn find_gamma() {
        let expected_result = ("10110".to_string(),"0".to_string());
        let actual_result = get_gamma_and_epsilon(get_test_input());
        assert_eq!(actual_result, expected_result)
    }
}
