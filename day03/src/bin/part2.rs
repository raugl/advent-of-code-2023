fn process(input: &str) -> i32 {
    let lines: Vec<_> = input
        .lines()
        .filter_map(|line| match !line.is_empty() {
            true => Some(line.trim().as_bytes()),
            false => None,
        })
        .collect();

    lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter_map(|(x, ch)| (*ch == b'*').then(|| (x, y)))
                .collect::<Vec<_>>()
        })
        .flatten()
        .filter_map(|coord| {
            let nums_coords = get_nums_coords(coord, &lines);
            if nums_coords.len() == 2 {
                Some(get_num(nums_coords[0], &lines) * get_num(nums_coords[1], &lines))
            } else {
                None
            }
        })
        .sum()
}

fn get_nums_coords(coord: (usize, usize), lines: &[&[u8]]) -> Vec<(usize, usize)> {
    let (width, height) = (lines.first().unwrap().len(), lines.len());
    let (coord_x, coord_y) = (coord.0, coord.1);
    let mut num_coords = Vec::new();

    for y in coord_y.saturating_sub(1)..(coord_y + 2).min(height) {
        let mut found_digit = false;

        for x in coord_x.saturating_sub(1)..(coord_x + 2).min(width) {
            if (lines[y][x] as char).is_ascii_digit() {
                if found_digit == false {
                    num_coords.push((x, y));
                    found_digit = true;
                }
            } else {
                found_digit = false;
            }
        }
    }
    num_coords
}

fn get_num(coord: (usize, usize), lines: &Vec<&[u8]>) -> i32 {
    let (coord_x, coord_y) = (coord.0, coord.1);
    let (mut start, mut end) = (coord_x, coord_x);
    let line = lines[coord_y];

    while start > 0 && (line[start - 1] as char).is_ascii_digit() {
        start -= 1;
    }
    while end < line.len() && (line[end] as char).is_ascii_digit() {
        end += 1;
    }
    std::str::from_utf8(&line[start..end])
        .ok()
        .and_then(|num| num.parse::<i32>().ok())
        .unwrap()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_num_coords() {
        let inputs = [
            ("...\n.*.\n...\n", 0),
            ("..1\n.*.\n...\n", 1),
            (".1.\n.*.\n...\n", 1),
            (".11\n.*.\n...\n", 1),
            ("1..\n.*.\n...\n", 1),
            ("1.1\n.*.\n...\n", 2),
            ("11.\n.*.\n...\n", 1),
            ("111\n.*.\n...\n", 1),
            ("...\n.*1\n...\n", 1),
            ("...\n1*.\n...\n", 1),
            ("...\n1*1\n...\n", 2),
        ];

        for (i, &(lines, expected_count)) in inputs.iter().enumerate() {
            let lines = lines
                .lines()
                .filter_map(|line| match !line.is_empty() {
                    true => Some(line.trim().as_bytes()),
                    false => None,
                })
                .collect::<Vec<_>>();

            assert_eq!(
                get_nums_coords((1, 1), &lines).len(),
                expected_count,
                "Test {} failed",
                i,
            );
        }
    }

    #[test]
    fn test_process() {
        let input = "
            ......552.../33.
            ................
            401....50.$.....
            ........#.566...
            307...........41
            .....281........
            .972.......406*8
            ....*..960......
            .777...=...811..
            ..............@.
            ................
            2...662....816.6
            ...*.......*....
            788.........186.
            ....../.........
            ......529.......
        ";
        assert_eq!(process(input), 1431924);
    }
}
