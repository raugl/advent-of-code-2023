use std::ops::Range;

fn process(input: &str) -> i32 {
    let lines: Vec<&str> = input
        .lines()
        .filter_map(|line| match !line.is_empty() {
            true => Some(line.trim()),
            false => None,
        })
        .collect();

    lines
        .iter()
        .enumerate()
        .map(|(row, line)| {
            let mut sum = 0;
            let mut start = 0;

            while let Some(new_start) = find_after(line, start, |ch| ch.is_digit(10)) {
                start = new_start;
                let end = find_after(line, start, |ch| !ch.is_digit(10)).unwrap_or(line.len());
                let num_str = &line[start..end];

                if is_part_num(row, start..end, &lines) {
                    sum += num_str.parse::<i32>().expect("Failed to parse part number");
                }
                start = end;
            }
            sum
        })
        .sum()
}

fn find_after(input: &str, pos: usize, pat: impl Fn(char) -> bool) -> Option<usize> {
    input
        .get(pos..)
        .and_then(|after_str| after_str.find(pat))
        .and_then(|end| Some(pos + end))
}

fn is_part_num(row: usize, cols: Range<usize>, lines: &Vec<&str>) -> bool {
    if cols.is_empty() {
        return false;
    }

    let lines = &lines[expand_range(row..row + 1, 0..lines.len())];
    let cols_range = expand_range(cols, 0..lines[0].len());

    lines
        .iter()
        .flat_map(|&line| line[cols_range.clone()].chars())
        .find(|ch| !ch.is_digit(10) && *ch != '.')
        .is_some()
}

fn expand_range(range: Range<usize>, valid_range: Range<usize>) -> Range<usize> {
    let (start, end) = (range.start as isize, range.end as isize);
    let (min, max) = (valid_range.start as isize, valid_range.end as isize);

    ((start - 1).max(min) as usize)..((end + 1).min(max) as usize)
}

fn main() {
    let input = include_str!("./input.txt");
    println!("result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
            467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..
        ";
        assert_eq!(process(input), 4361);
    }
}
