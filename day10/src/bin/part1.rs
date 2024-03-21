use std::collections::{HashSet, VecDeque};

fn process(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let mut queue = VecDeque::from([find_start(&lines)]);
    let mut visited = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        if visited.insert(pos) {
            if let Some(neighbours) = get_neighbours(pos, &lines) {
                queue.extend(neighbours);
            }
        }
    }

    (visited.len() as i64 + 1) / 2
}

fn find_start(lines: &Vec<&str>) -> (usize, usize) {
    let (x, y) = (|| {
        for (y, line) in lines.iter().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                if ch == 'S' {
                    return (x, y);
                }
            }
        }
        unreachable!("There is no starting position")
    })();

    let neighbours = [
        (x, y.saturating_add(1)),
        (x, y.saturating_sub(1)),
        (x.saturating_sub(1), y),
        (x.saturating_add(1), y),
    ];

    for neighbour in neighbours {
        if let Some(neighbours) = get_neighbours(neighbour, &lines) {
            if neighbours
                .iter()
                .find(|neighbour| **neighbour == (x, y))
                .is_some()
            {
                return neighbour;
            }
        }
    }
    unreachable!("Starting position has no neighbours")
}

fn get_neighbours((x, y): (usize, usize), lines: &Vec<&str>) -> Option<[(usize, usize); 2]> {
    if y >= lines.len() || x >= lines.first().unwrap().len() {
        return None;
    }

    let up = (x, y.saturating_sub(1));
    let down = (x, y.saturating_add(1));
    let left = (x.saturating_sub(1), y);
    let right = (x.saturating_add(1), y);

    let tile = lines[y].chars().skip(x).next().unwrap();
    let neighbours = match tile {
        '|' | '│' => Some([up, down]),
        '-' | '─' => Some([left, right]),
        'L' | '╰' => Some([up, right]),
        'J' | '╯' => Some([up, left]),
        '7' | '╮' => Some([down, left]),
        'F' | '╭' => Some([down, right]),
        _ => None,
    };
    neighbours
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
            .....
            .S─╮.
            .│.│.
            .╰─╯.
            .....
        ";
        assert_eq!(process(input), 4);

        let input = "
            ─╰│╭╮
            ╮S─╮│
            ╰│╮││
            ─╰─╯│
            ╰│─╯╭
        ";
        assert_eq!(process(input), 4);

        let input = "
            ..╭╮.
            .╭╯│.
            S╯.╰╮
            │╭──╯
            ╰╯...
        ";
        assert_eq!(process(input), 8);

        let input = "
            ╮─╭╮─
            .╭╯│╮
            S╯╰╰╮
            │╭──╯
            ╰╯.╰╯
        ";
        assert_eq!(process(input), 8);
    }
}
