use std::cmp;

fn process(input: &str) -> i64 {
    let mut lines = input.lines().map(|line| line.trim());
    let mut result = 0;

    while let Some(lines) = parse_block(&mut lines) {
        let row = (1..lines.len()).find(|y| test_row(&lines, *y));

        if let Some(row) = row {
            result += row * 100;
        } else {
            let width = lines.first().map_or(0, |line| line.len());
            result += (1..width).find(|x| test_column(&lines, *x)).unwrap();
        }
    }

    result as i64
}

fn parse_block<'a>(input: &mut impl Iterator<Item = &'a str>) -> Option<Vec<&'a str>> {
    let mut lines = Vec::new();

    while let Some(line) = input.next() {
        match (line.is_empty(), lines.is_empty()) {
            (true, false) => break,
            (true, true) => continue,
            (false, _) => lines.push(line),
        }
    }

    (!lines.is_empty()).then_some(lines)
}

fn test_row(lines: &Vec<&str>, y: usize) -> bool {
    let half_count = cmp::min(y, lines.len().saturating_sub(y));
    let top = &lines[y - half_count..y];
    let bot = &lines[y..y + half_count];

    top.iter().rev().eq(bot.iter())
}

fn test_column(lines: &Vec<&str>, x: usize) -> bool {
    lines.iter().all(|line| {
        let half_count = cmp::min(x, line.len().saturating_sub(x));
        let left = &line[x - half_count..x];
        let right = &line[x..x + half_count];

        left.chars().rev().eq(right.chars())
    })
}

fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
            #.##..##.
            ..#.##.#.
            ##......#
            ##......#
            ..#.##.#.
            ..##..##.
            #.#.##.#.

            #...##..#
            #....#..#
            ..##..###
            #####.##.
            #####.##.
            ..##..###
            #....#..#
        ";
        assert_eq!(process(input), 405);
    }
}
