use std::collections::{BTreeSet, VecDeque};
use std::mem;

fn process(input: &str) -> i64 {
    let mut bricks = parse_bricks(input);
    let mut children = vec![BTreeSet::new(); bricks.len()];
    let mut parents = vec![BTreeSet::new(); bricks.len()];

    bricks.sort_unstable_by_key(|(start, _end)| start[2]);
    for i in 0..bricks.len() {
        let min_z = bricks
            .iter()
            .take(i)
            .filter(|lower| overlaps(&bricks[i], lower))
            .map(|(_start, end)| end[2])
            .max()
            .unwrap_or(0);

        let (start, end) = &mut bricks[i];
        end[2] -= start[2] - (min_z + 1);
        start[2] = min_z + 1;
    }

    bricks.sort_unstable_by_key(|(start, _end)| start[2]);
    for (upper_idx, upper) in bricks.iter().enumerate() {
        for (lower_idx, lower) in bricks.iter().enumerate().take(upper_idx) {
            if overlaps(upper, lower) && upper.0[2] == lower.1[2] + 1 {
                children[lower_idx].insert(upper_idx);
                parents[upper_idx].insert(lower_idx);
            }
        }
    }

    let mut count = 0;
    for i in 0..bricks.len() {
        let mut queue = VecDeque::<usize>::new();
        let mut falling = vec![false; parents.len()];

        falling[i] = true;
        queue.extend(children[i].iter());

        while let Some(node) = queue.pop_front() {
            if !falling[node] && parents[node].iter().all(|parent| falling[*parent]) {
                count += 1;
                falling[node] = true;
                queue.extend(children[node].iter());
            }
        }
    }
    count
}

fn overlaps(a: &(Vec<usize>, Vec<usize>), b: &(Vec<usize>, Vec<usize>)) -> bool {
    a.0[0] <= b.1[0] && b.0[0] <= a.1[0] && a.0[1] <= b.1[1] && b.0[1] <= a.1[1]
}

fn parse_bricks(input: &str) -> Vec<(Vec<usize>, Vec<usize>)> {
    input
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
        .collect()
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
        assert_eq!(process(input), 7);
    }
}
