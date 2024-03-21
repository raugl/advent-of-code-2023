use std::collections::{BTreeMap, HashSet, VecDeque};

fn process(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let pipe_tiles = get_pipe_tiles(&lines);
    let mut visited = pipe_tiles.clone();

    let height = lines.len();
    let width = lines.first().unwrap().chars().count();

    (0..height)
        .flat_map(|y| (0..width).map(move |x| (x, y)))
        .map(|pos| {
            if !visited.contains(&pos) {
                let fill_tiles = flood_fill(pos, &mut visited, &lines);
                if is_inside(pos, &pipe_tiles, &lines) {
                    return fill_tiles.len();
                }
            }
            return 0;
        })
        .sum()
}

fn flood_fill(
    pos: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
    lines: &Vec<&str>,
) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::from([pos]);
    let mut vec = Vec::new();

    while let Some(pos) = queue.pop_front() {
        if visited.insert(pos) {
            let (x, y) = pos;
            let neighbours = [
                (x, y.saturating_add(1)),
                (x, y.saturating_sub(1)),
                (x.saturating_sub(1), y),
                (x.saturating_add(1), y),
            ]
            .into_iter()
            .filter(|&(x, y)| y < lines.len() && x < lines.first().unwrap().chars().count());

            vec.push(pos);
            queue.extend(neighbours);
        }
    }
    vec
}

fn to_rounded(ch: char) -> char {
    if ['.', '│', '─', '╰', '╯', '╭', '╮'].contains(&ch) {
        return ch;
    }

    match ch {
        '|' => '│',
        '-' => '─',
        'L' => '╰',
        'J' => '╯',
        'F' => '╭',
        '7' => '╮',
        'S' => '╭', // HACK:
        _ => panic!("Unexpected symbol: {ch}"),
    }
}

fn is_inside(pos: (usize, usize), pipe_tiles: &HashSet<(usize, usize)>, lines: &Vec<&str>) -> bool {
    let ray_symbols = lines[pos.1]
        .chars()
        .enumerate()
        .take(pos.0)
        .map(|(x, ch)| ((x, pos.1), ch))
        .filter(|(pos, _ch)| pipe_tiles.contains(pos))
        .map(|(_pos, ch)| ch)
        .collect::<Vec<_>>();

    let mut counts = BTreeMap::new();
    for symbol in ['│', '╰', '╯', '╭', '╮'] {
        let count = ray_symbols
            .iter()
            .filter(|ch| to_rounded(**ch) == symbol)
            .count();
        counts.insert(symbol, count);
    }

    let pair_count1 = counts[&'╭'].min(counts[&'╮']);
    let pair_count2 = counts[&'╰'].min(counts[&'╯']);
    let max_count = counts[&'╰'].max(counts[&'╯']);
    (pair_count1 * 2 + pair_count2 + max_count + counts[&'│']) % 2 == 1
}

fn get_pipe_tiles(lines: &Vec<&str>) -> HashSet<(usize, usize)> {
    let mut queue = VecDeque::from([find_start(&lines)]);
    let mut visited = HashSet::new();

    while let Some(pos) = queue.pop_front() {
        if visited.insert(pos) {
            if let Some(neighbours) = get_neighbours(pos, &lines) {
                queue.extend(neighbours);
            }
        }
    }
    visited
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
    if y >= lines.len() || x >= lines.first().unwrap().chars().count() {
        return None;
    }

    let up = (x, y.saturating_sub(1));
    let down = (x, y.saturating_add(1));
    let left = (x.saturating_sub(1), y);
    let right = (x.saturating_add(1), y);

    let tile = lines[y].chars().skip(x).next().unwrap();
    let neighbours = match to_rounded(tile) {
        '│' => Some([up, down]),
        '─' => Some([left, right]),
        '╰' => Some([up, right]),
        '╯' => Some([up, left]),
        '╮' => Some([down, left]),
        '╭' => Some([down, right]),
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
            ...........
            .S───────╮.
            .│╭─────╮│.
            .││.....││.
            .││.....││.
            .│╰─╮.╭─╯│.
            .│..│.│..│.
            .╰──╯.╰──╯.
            ...........
        ";
        assert_eq!(process(input), 4);

        let input = "
            .╭────╮╭╮╭╮╭╮╭─╮....
            .│╭──╮││││││││╭╯....
            .││.╭╯││││││││╰╮....
            ╭╯╰╮╰╮╰╯╰╯││╰╯.╰─╮..
            ╰──╯.╰╮...╰╯S╮╭─╮╰╮.
            ....╭─╯..╭╮╭╯│╰╮╰╮╰╮
            ....╰╮.╭╮││╰╮│.╰╮╰╮│
            .....│╭╯╰╯│╭╯│╭╮│.╰╯
            ....╭╯╰─╮.││.││││...
            ....╰───╯.╰╯.╰╯╰╯...
        ";
        assert_eq!(process(input), 8);

        let input = "
            ╭╭╮╭S╭╮╭╮╭╮╭╮╭╮╭───╮
            ╰│╰╯││││││││││││╭──╯
            ╭╰─╮╰╯╰╯││││││╰╯╰─╮╮
            ╭──╯╭──╮││╰╯╰╯╮╭╮╭╯─
            ╰───╯╭─╯╰╯.││─╭╯╰╯╯╮
            │╭│╭─╯╭───╮╭╮─╰╮╰│╮│
            │╭╭╯╭╮╰╮╭─╯╭╮│╯╰───╮
            ╮─╰─╯╰╮││╭╮│╰╮╭─╮╭╮│
            ╰.╰╮╰╭╯│││││╭╯╰╮││╰╯
            ╰╮╯╰╯╰─╯╰╯╰╯╰──╯╰╯.╰
        ";
        assert_eq!(process(input), 10);
    }
}
