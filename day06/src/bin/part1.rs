fn process(input: &str) -> i64 {
    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let time_records = parse_line(&mut lines)
        .into_iter()
        .zip(parse_line(&mut lines).into_iter());

    time_records
        .map(|(time, record)| {
            (1..time)
                .map(|wait_time| wait_time * (time - wait_time))
                .filter(|dist| *dist > record)
                .count() as i64
        })
        .product()
}

fn parse_line<'a>(lines: &mut impl Iterator<Item = &'a str>) -> Vec<i64> {
    lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .flat_map(|num| num.parse())
        .collect()
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
        assert_eq!(process(input), 288);
    }
}
