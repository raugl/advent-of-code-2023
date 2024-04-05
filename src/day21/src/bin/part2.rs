use std::collections::VecDeque;

fn process(input: &str, steps: i64) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let side = {
        let height = grid.len() as i64;
        let width = grid.first().unwrap().len() as i64;

        assert!(width == height, "[ERROR] Expected a square grid as input");
        width
    };

    let whole_radius = steps / side - 1;
    let start = Node {
        x: side / 2,
        y: side / 2,
        depth: 0,
    };

    assert!(
        side % 2 == 1,
        "[ERROR] Expected the side lenght of the input to be odd"
    );
    assert!(
        grid[start.y as usize][start.x as usize] == 'S',
        "[ERROR] Expected the starting point to ben in the middle of the grid"
    );

    let full_even = {
        let radius = whole_radius - (whole_radius % 2 == 1) as i64;
        let even_rings_area = radius * (radius + 2) + 1;
        count_reachable(start, steps, &grid) * even_rings_area
    };

    let full_odd = {
        let radius = whole_radius - (whole_radius % 2 == 0) as i64;
        let odd_rings_area = (radius + 1).pow(2);
        count_reachable(start, steps + 1, &grid) * odd_rings_area
    };

    let remaining_steps = steps - whole_radius * side - side / 2;
    let corners: i64 = [
        Node::new(0, side / 2, 1),        // Left
        Node::new(side / 2, 0, 1),        // Up
        Node::new(side - 1, side / 2, 1), // Right
        Node::new(side / 2, side - 1, 1), // Down
    ]
    .into_iter()
    .map(|start| {
        let steps = remaining_steps + (whole_radius % 2 == 0) as i64;
        count_reachable(start, steps, &grid)
    })
    .sum();

    let deep_edges: i64 = [
        Node::new(0, 0, 1),               // Upper-Left
        Node::new(side - 1, 0, 1),        // Upper-Right
        Node::new(0, side - 1, 1),        // Lower-Left
        Node::new(side - 1, side - 1, 1), // Lower-Right
    ]
    .into_iter()
    .map(|start| {
        let steps = remaining_steps + side / 2 + (whole_radius % 2 == 0) as i64;
        count_reachable(start, steps, &grid) * whole_radius
    })
    .sum();

    let shallow_edges: i64 = [
        Node::new(0, 0, 1),               // Upper-Left
        Node::new(side - 1, 0, 1),        // Upper-Right
        Node::new(0, side - 1, 1),        // Lower-Left
        Node::new(side - 1, side - 1, 1), // Lower-Right
    ]
    .into_iter()
    .map(|start| {
        let steps = remaining_steps - (side / 2 + 1) + (whole_radius % 2 == 0) as i64;
        count_reachable(start, steps, &grid) * (whole_radius + 1)
    })
    .sum();

    full_even + full_odd + corners + deep_edges + shallow_edges
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

fn count_reachable(start: Node, steps: i64, grid: &Vec<Vec<char>>) -> i64 {
    let height = grid.len();
    let width = grid.first().unwrap().len();

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

fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}", process(input, 26_501_365));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
            .............
            .#.........#.
            .............
            .............
            .............
            .............
            ......S......
            .............
            .............
            .............
            .............
            .#.........#.
            .............
        ";
        assert_eq!(process(input, 29), 876);
    }
}
