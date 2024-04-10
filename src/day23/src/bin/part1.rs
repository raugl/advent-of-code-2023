use std::collections::HashMap;

fn process(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let nodes = get_collapsed_nodes(&grid);
    let adjacency = get_adjacency(&nodes, &grid);
    let order = topological_order(&adjacency);

    let mut dists = vec![0; nodes.len()];
    for curr in order {
        for (next, dist) in adjacency[curr].iter().enumerate() {
            if let Some(dist) = dist {
                dists[next] = dists[next].max(dists[curr] + dist);
            }
        }
    }

    *dists.last().unwrap() as i64
}

fn topological_order(adjacency: &Vec<Vec<Option<i32>>>) -> Vec<usize> {
    fn dfs(curr: usize, adjacency: &Vec<Vec<Option<i32>>>, visited: &mut Vec<bool>) -> Vec<usize> {
        visited[curr] = true;
        adjacency[curr]
            .iter()
            .enumerate()
            .filter_map(|(next, dist)| dist.and(Some(next)))
            .flat_map(|next| match visited[next] {
                true => vec![],
                false => dfs(next, adjacency, visited),
            })
            .chain([curr])
            .collect()
    }

    let mut visited = vec![false; adjacency.len()];
    let mut order = dfs(0, adjacency, &mut visited);
    order.reverse();
    order
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Hash, Clone, Default, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Vec2 { x, y }
    }
}

fn get_collapsed_nodes(grid: &Vec<Vec<char>>) -> Vec<Vec2> {
    let find_path = |y| Vec2 {
        x: grid[y as usize].iter().position(|ch| *ch == '.').unwrap() as i32,
        y: y as i32,
    };

    let (width, height) = (grid.first().unwrap().len() as i32, grid.len() as i32);
    let (start, end) = (find_path(0), find_path(height - 1));

    let junctions = (0..height)
        .flat_map(|y| (0..width).map(move |x| Vec2 { x, y }))
        .filter(|pos| {
            let (width, height) = (grid.first().unwrap().len(), grid.len());
            let count = [
                Vec2::new(pos.x - 1, pos.y),
                Vec2::new(pos.x + 1, pos.y),
                Vec2::new(pos.x, pos.y - 1),
                Vec2::new(pos.x, pos.y + 1),
            ]
            .into_iter()
            .filter(|pos| {
                (0..width as i32).contains(&pos.x)
                    && (0..height as i32).contains(&pos.y)
                    && *get(grid, pos) != '#'
            })
            .count();
            *get(grid, pos) != '#' && count > 2
        });

    [start.clone()]
        .into_iter()
        .chain(junctions)
        .chain([end.clone()])
        .collect()
}

fn get_adjacency(nodes: &Vec<Vec2>, grid: &Vec<Vec<char>>) -> Vec<Vec<Option<i32>>> {
    let nodes: HashMap<Vec2, usize> = nodes.iter().cloned().zip(0..).collect();
    let (width, height) = (grid.first().unwrap().len(), grid.len());
    let mut adjacency = vec![vec![None; nodes.len()]; nodes.len()];

    for (start, start_idx) in nodes.iter() {
        let mut stack = vec![(0, start.clone())];
        let visited = &mut vec![vec![false; width]; height];
        *get_mut(visited, start) = true;

        while let Some((dist, node)) = stack.pop() {
            if node != *start && nodes.contains_key(&node) {
                let end_idx = nodes.get(&node).unwrap();
                adjacency[*start_idx][*end_idx] = Some(dist);
                continue;
            }

            for next in neighbours(&node, grid) {
                if !get(visited, &next) {
                    *get_mut(visited, &node) = true;
                    stack.push((dist + 1, next));
                }
            }
        }
    }
    adjacency
}

fn neighbours(pos: &Vec2, grid: &Vec<Vec<char>>) -> Vec<Vec2> {
    let (width, height) = (grid.first().unwrap().len(), grid.len());
    match get(grid, pos) {
        '.' => vec![
            Vec2::new(pos.x - 1, pos.y),
            Vec2::new(pos.x + 1, pos.y),
            Vec2::new(pos.x, pos.y - 1),
            Vec2::new(pos.x, pos.y + 1),
        ],
        '<' => vec![Vec2::new(pos.x - 1, pos.y)],
        '>' => vec![Vec2::new(pos.x + 1, pos.y)],
        '^' => vec![Vec2::new(pos.x, pos.y - 1)],
        'v' => vec![Vec2::new(pos.x, pos.y + 1)],
        _ => vec![],
    }
    .into_iter()
    .filter(|pos| {
        (0..width as i32).contains(&pos.x)
            && (0..height as i32).contains(&pos.y)
            && *get(grid, pos) != '#'
    })
    .collect()
}

fn get<'a, T>(grid: &'a Vec<Vec<T>>, pos: &Vec2) -> &'a T {
    &grid[pos.y as usize][pos.x as usize]
}

fn get_mut<'a, T>(grid: &'a mut Vec<Vec<T>>, pos: &Vec2) -> &'a mut T {
    &mut grid[pos.y as usize][pos.x as usize]
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
            #.#####################
            #.......#########...###
            #######.#########.#.###
            ###.....#.>.>.###.#.###
            ###v#####.#v#.###.#.###
            ###.>...#.#.#.....#...#
            ###v###.#.#.#########.#
            ###...#.#.#.......#...#
            #####.#.#.#######.#.###
            #.....#.#.#.......#...#
            #.#####.#.#.#########v#
            #.#...#...#...###...>.#
            #.#.#v#######v###.###v#
            #...#.>.#...>.>.#.###.#
            #####v#.#.###v#.#.###.#
            #.....#...#...#.#.#...#
            #.#########.###.#.#.###
            #...###...#...#...#.###
            ###.###.#.###v#####v###
            #...#...#.#.>.>.#.>.###
            #.###.###.#.###.#.#v###
            #.....###...###...#...#
            #####################.#
        ";
        assert_eq!(process(input), 94);
    }
}
