use std::collections::VecDeque;

fn process(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    use Direction::*;
    let height = grid.len();
    let width = grid.first().map_or(0, |row| row.len());

    let mut visited = vec![vec![0; width]; height];
    let mut queue = VecDeque::from([Node::new(0, 0, Right)]);

    while let Some(node) = queue.pop_front() {
        let (x, y) = (node.x as usize, node.y as usize);
        let ch = grid[y][x];
        visited[y][x] |= node.dir as u8;

        queue.extend(
            node.neighbours(ch)
                .into_iter()
                .filter(|&Node { x, y, dir }| {
                    (0 <= x && x < width as i64 && 0 <= y && y < height as i64)
                        && visited[y as usize][x as usize] & dir as u8 == 0
                }),
        );
    }

    visited
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|visited| *visited != 0)
        .count() as i64
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum Direction {
    Up = 0b0001,
    Down = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

#[derive(Clone)]
struct Node {
    x: i64,
    y: i64,
    dir: Direction,
}

impl Node {
    fn new(x: i64, y: i64, dir: Direction) -> Self {
        Self { x, y, dir }
    }

    fn neighbours(&self, ch: char) -> Vec<Node> {
        use Direction::*;

        let Node { x, y, dir } = self.clone();
        let neighbour = |dir| {
            let (x, y) = match dir {
                Up => (x, y - 1),
                Down => (x, y + 1),
                Left => (x - 1, y),
                Right => (x + 1, y),
            };
            Node { x, y, dir }
        };

        match (ch, dir) {
            ('.', dir) => vec![neighbour(dir)],
            ('-', dir) => match dir {
                Left => vec![neighbour(Left)],
                Right => vec![neighbour(Right)],
                Up | Down => vec![neighbour(Left), neighbour(Right)],
            },
            ('|', dir) => match dir {
                Up => vec![neighbour(Up)],
                Down => vec![neighbour(Down)],
                Left | Right => vec![neighbour(Up), neighbour(Down)],
            },
            ('/', dir) => match dir {
                Up => vec![neighbour(Right)],
                Down => vec![neighbour(Left)],
                Left => vec![neighbour(Down)],
                Right => vec![neighbour(Up)],
            },
            ('\\', dir) => match dir {
                Up => vec![neighbour(Left)],
                Down => vec![neighbour(Right)],
                Left => vec![neighbour(Up)],
                Right => vec![neighbour(Down)],
            },
            _ => unreachable!(),
        }
    }
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
        let input = r#"
            .|...\....
            |.-.\.....
            .....|-...
            ........|.
            ..........
            .........\
            ..../.\\..
            .-.-/..|..
            .|....-|.\
            ..//.|....
        "#;
        assert_eq!(process(input), 46);
    }
}
