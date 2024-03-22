use std::iter;

fn process(input: &str) -> i32 {
    let digits = [
        ("0", 0),
        ("1", 1),
        ("one", 1),
        ("2", 2),
        ("two", 2),
        ("3", 3),
        ("three", 3),
        ("4", 4),
        ("four", 4),
        ("5", 5),
        ("five", 5),
        ("6", 6),
        ("six", 6),
        ("7", 7),
        ("seven", 7),
        ("8", 8),
        ("eight", 8),
        ("9", 9),
        ("nine", 9),
    ];

    input
        .lines()
        .filter_map(|line| {
            let matches = digits
                .iter()
                .flat_map(|(pattern, value)| line.match_indices(pattern).zip(iter::repeat(value)))
                .map(|((pos, _match), value)| (pos, value));

            let first_digit = matches.clone().min_by(|a, b| a.0.cmp(&b.0))?.1;
            let last_digit = matches.clone().max_by(|a, b| a.0.cmp(&b.0))?.1;
            Some(first_digit * 10 + last_digit)
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = r#"
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "#;
        assert_eq!(process(input), 281);
    }
}
