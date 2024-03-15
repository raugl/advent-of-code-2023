fn process(input: &str) -> i64 {
    input
        .lines()
        .flat_map(|line| {
            let first_digit = line.chars().find(|ch| ch.is_digit(10))?;
            let last_digit = line.chars().rfind(|ch| ch.is_digit(10))?;
            format!("{}{}", first_digit, last_digit).parse::<i64>().ok()
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
        let input = "
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
        ";
        assert_eq!(process(input), 142);
    }
}
