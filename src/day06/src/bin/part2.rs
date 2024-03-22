fn process(input: &str) -> i64 {
    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let (time, record) = (parse_line(&mut lines), parse_line(&mut lines));
    (1..time)
        .map(|wait_time| wait_time * (time - wait_time))
        .filter(|dist| *dist > record)
        .count() as i64
}

fn parse_line<'a>(lines: &mut impl Iterator<Item = &'a str>) -> i64 {
    lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .flat_map(|num| num.chars())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("Result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
        Time:      7  15   30
        Distance:  9  40  200
        ";
        assert_eq!(process(input), 71503);
    }
}

