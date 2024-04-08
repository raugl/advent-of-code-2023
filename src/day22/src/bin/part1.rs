use std::{collections::BTreeSet, mem};

fn process(input: &str) -> i64 {
    let mut bricks = parse_bricks(input);
    let tower = get_tower(&mut bricks);

    let nodes: Vec<Node> = bricks
        .iter()
        .map(|(start, end)| {
            let mut node = Node::default();

            for x in start[0]..=end[0] {
                for y in start[1]..=end[1] {
                    if let Some(parent) = tower[start[2] - 1][y][x] {
                        node.parents.insert(parent);
                    }
                    if let Some(child) = tower[end[2] + 1][y][x] {
                        node.children.insert(child);
                    }
                }
            }
            node
        })
        .collect();

    nodes
        .iter()
        .filter(|node| {
            node.children
                .iter()
                .map(|child| nodes[*child].parents.len())
                .all(|parents_len| parents_len > 1)
        })
        .count() as i64
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default, Debug)]
struct Node {
    parents: BTreeSet<usize>,
    children: BTreeSet<usize>,
}

fn parse_bricks(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    let mut bricks: Vec<(Vec<usize>, Vec<usize>)> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (start, end) = line.split_once('~').unwrap();
            let mut start: Vec<usize> = start.split(',').map(|num| num.parse().unwrap()).collect();
            let mut end: Vec<usize> = end.split(',').map(|num| num.parse().unwrap()).collect();

            let sort = |a: &mut usize, b: &mut usize| match a > b {
                true => mem::swap(a, b),
                false => (),
            };
            sort(&mut start[0], &mut end[0]);
            sort(&mut start[1], &mut end[1]);
            sort(&mut start[2], &mut end[2]);

            (start, end)
        })
        .collect();

    bricks.sort_unstable_by_key(|(start, _end)| start[2]);
    bricks
}

fn get_tower(bricks: &mut Vec<(Vec<usize>, Vec<usize>)>) -> Vec<Vec<Vec<Option<usize>>>> {
    let max_height = bricks.iter().map(|(_start, end)| end[2]).max().unwrap();

    let mut max_heights = vec![vec![0; 10]; 10];
    let mut tower = vec![vec![vec![None; 10]; 10]; max_height + 1];

    for (id, (start, end)) in bricks.iter_mut().enumerate() {
        let mut min_z = 0;
        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                min_z = min_z.max(max_heights[y][x]);
            }
        }
        let dz = start[2] - (min_z + 1);
        start[2] -= dz;
        end[2] -= dz;

        for x in start[0]..=end[0] {
            for y in start[1]..=end[1] {
                for z in start[2]..=end[2] {
                    tower[z][y][x] = Some(id);
                }
                max_heights[y][x] = end[2];
            }
        }
    }
    tower
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
            1,0,1~1,2,1
            0,0,2~2,0,2
            0,2,3~2,2,3
            0,0,4~0,2,4
            2,0,5~2,2,5
            0,1,6~2,1,6
            1,1,8~1,1,9
        ";
        assert_eq!(process(input), 5);
    }
}
