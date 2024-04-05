use std::collections::VecDeque;

fn process(input: &str, steps: i64) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len();
    let width = grid.first().unwrap().len();
    let start = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(move |(x, ch)| ((x as i64, y as i64), ch))
        })
        .find(|(_pos, ch)| **ch == 'S')
        .map(|((x, y), _ch)| Node { x, y, depth: 0 })
        .unwrap();

    let mut queue = VecDeque::from([start]);
    let mut visited = vec![vec![false; width]; height];
    let mut count = 0;

    while let Some(node) = queue.pop_front() {
        if visited[node.y as usize][node.x as usize] == true {
            continue;
        }

        if node.depth % 2 == steps % 2 {
            count += 1;
        }

        let neighbours = [
            Node::new(node.x, node.y - 1, node.depth + 1),
            Node::new(node.x, node.y + 1, node.depth + 1),
            Node::new(node.x - 1, node.y, node.depth + 1),
            Node::new(node.x + 1, node.y, node.depth + 1),
        ]
        .into_iter()
        .filter(|node| {
            node.depth <= steps
                && (0..width as i64).contains(&node.x)
                && (0..height as i64).contains(&node.y)
                && ['.', 'S'].contains(&grid[node.y as usize][node.x as usize])
        });

        queue.extend(neighbours);
        visited[node.y as usize][node.x as usize] = true;
    }
    count
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Default, Debug)]
struct Node {
    x: i64,
    y: i64,
    depth: i64,
}

impl Node {
    fn new(x: i64, y: i64, depth: i64) -> Self {
        Self { x, y, depth }
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}", process(input, 64));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
            ...........
            .....###.#.
            .###.##..#.
            ..#.#...#..
            ....#.#....
            .##..S####.
            .##..#...#.
            .......##..
            .##.#.####.
            .##..##.##.
            ...........
        ";
        assert_eq!(process(input, 6), 16);
    }
}
