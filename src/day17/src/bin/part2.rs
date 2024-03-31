use std::collections::{BinaryHeap, HashMap};

fn process(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let height = grid.len() as i32;
    let width = grid.first().unwrap().len() as i32;
    a_star(Vec2::new(0, 0), Vec2::new(width - 1, height - 1), grid).unwrap()
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Hash, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
    Start,
}

#[derive(PartialEq, Eq, Clone, Hash, Debug)]
struct DirRun {
    dir: Direction,
    len: i32,
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Node {
    pos: Vec2,
    dist: i32,
    run: DirRun,
}

fn a_star(start: Vec2, end: Vec2, grid: Vec<Vec<char>>) -> Option<i32> {
    use Direction::*;
    let start = Node {
        pos: start,
        dist: 0,
        run: DirRun { dir: Start, len: 1 },
    };

    let mut heap = BinaryHeap::from([start.clone()]);
    let mut dists = HashMap::from([((start.pos, start.run), start.dist)]);
    let heuristic = |node: Vec2| (end.x - node.x).abs() + (end.y - node.y).abs();

    while let Some(curr) = heap.pop() {
        let curr_key = (curr.pos.clone(), curr.run.clone());
        if curr.pos == end && 4 <= curr.run.len {
            return dists.get(&curr_key).cloned();
        }

        for next in neighbours(&curr, &grid) {
            let new_key = (next.pos.clone(), next.run.clone());
            let new_dist = dists[&curr_key] + next.dist;

            if !dists.contains_key(&new_key) {
                dists.insert(new_key.clone(), new_dist);
                heap.push(Node {
                    pos: next.pos.clone(),
                    dist: new_dist + heuristic(next.pos),
                    run: next.run,
                });
            }
        }
    }
    None
}

fn neighbours(node: &Node, grid: &Vec<Vec<char>>) -> Vec<Node> {
    use Direction::*;
    let height = grid.len() as i32;
    let width = grid.first().unwrap().len() as i32;

    let up = Vec2::new(node.pos.x, node.pos.y - 1);
    let down = Vec2::new(node.pos.x, node.pos.y + 1);
    let left = Vec2::new(node.pos.x - 1, node.pos.y);
    let right = Vec2::new(node.pos.x + 1, node.pos.y);

    node.run
        .neighbours()
        .map(|run| match run.dir {
            Up => (&up, run),
            Down => (&down, run),
            Left => (&left, run),
            Right => (&right, run),
            Start => unreachable!(),
        })
        .filter(|(pos, _run)| 0 <= pos.x && pos.x < width && 0 <= pos.y && pos.y < height)
        .map(|(pos, run)| Node {
            pos: pos.clone(),
            dist: grid[pos.y as usize][pos.x as usize] as i32 - '0' as i32,
            run,
        })
        .collect()
}

impl DirRun {
    fn neighbours(&self) -> impl Iterator<Item = Self> {
        use Direction::*;
        let clone1 = self.clone();
        let clone2 = self.clone();
        let opposite = match self.dir {
            Up => Down,
            Down => Up,
            Left => Right,
            Right => Left,
            Start => Start,
        };

        [Up, Down, Left, Right]
            .into_iter()
            .map(move |dir| {
                let same_dir = clone1.dir == dir || clone1.dir == Start;
                let len = if same_dir { clone1.len + 1 } else { 1 };
                Self { dir, len }
            })
            .filter(move |run| {
                run.dir != opposite
                    && run.len <= 10
                    && (4 <= clone2.len || clone2.dir == run.dir || clone2.dir == Start)
            })
    }
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
        let input = "
            2413432311323
            3215453535623
            3255245654254
            3446585845452
            4546657867536
            1438598798454
            4457876987766
            3637877979653
            4654967986887
            4564679986453
            1224686865563
            2546548887735
            4322674655533
        ";
        assert_eq!(process(input), 94);

        let input = "
            111111111111
            999999999991
            999999999991
            999999999991
            999999999991
        ";
        assert_eq!(process(input), 71);
    }
}

