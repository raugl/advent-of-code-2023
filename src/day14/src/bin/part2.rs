use std::{
    collections::HashMap,
    hash::{DefaultHasher, Hash, Hasher},
};

fn process(input: &str) -> i64 {
    let mut lines: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let mut history = HashMap::new();
    for i in 0..1_000_000_000 {
        let hash = hash_grid(&lines);

        if let Some(start) = history.get(&hash) {
            let repeat_count = (1_000_000_000 - start) % (i - start);
            for _ in 0..4 * repeat_count {
                rotate_grid(&mut lines);
                roll_stones(&mut lines);
            }
            break;
        } else {
            history.insert(hash, i);
            for _ in 0..4 {
                rotate_grid(&mut lines);
                roll_stones(&mut lines);
            }
        }
    }

    let mut sum = 0;
    for (i, line) in lines.iter().rev().enumerate() {
        sum += (i + 1) * line.iter().filter(|ch| **ch == 'O').count();
    }
    sum as i64
}

fn roll_stones(lines: &mut Vec<Vec<char>>) {
    for line in lines {
        line.split_mut(|ch| *ch == '#')
            .filter(|slice| !slice.is_empty())
            .for_each(|slice| slice.sort_unstable());
    }
}

fn rotate_grid(lines: &mut Vec<Vec<char>>) {
    for y in 0..lines.len() {
        for x in y..lines.first().unwrap().len() {
            let temp = lines[x][y];
            lines[x][y] = lines[y][x];
            lines[y][x] = temp;
        }
    }

    for line in lines {
        line.reverse();
    }
}

fn hash_grid(lines: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    lines.hash(&mut hasher);
    hasher.finish()
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
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#....
        ";
        assert_eq!(process(input), 64);
    }

    #[test]
    fn test_cycle() {
        let mut lines: Vec<Vec<char>> = "
            O....#....
            O.OO#....#
            .....##...
            OO.#O....O
            .O.....O#.
            O.#..O.#.#
            ..O..#O..O
            .......O..
            #....###..
            #OO..#...."
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        cycle(&mut lines);
        assert_eq_grid(
            &lines,
            ".....#....
            ....#...O#
            ...OO##...
            .OO#......
            .....OOO#.
            .O#...O#.#
            ....O#....
            ......OOOO
            #...O###..
            #..OO#....",
        );

        cycle(&mut lines);
        assert_eq_grid(
            &lines,
            ".....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #..OO###..
            #.OOO#...O",
        );

        cycle(&mut lines);
        assert_eq_grid(
            &lines,
            ".....#....
            ....#...O#
            .....##...
            ..O#......
            .....OOO#.
            .O#...O#.#
            ....O#...O
            .......OOO
            #...O###.O
            #.OOO#...O",
        );
    }

    fn cycle(lines: &mut Vec<Vec<char>>) {
        for _ in 0..4 {
            rotate_grid(lines);
            roll_stones(lines);
        }
    }

    fn assert_eq_grid(lines: &Vec<Vec<char>>, input: &str) {
        let is_equal = {
            let input = input
                .lines()
                .map(|line| line.trim())
                .flat_map(|line| line.chars());

            let lines = lines.iter().flat_map(|line| line.iter()).cloned();
            input.eq(lines)
        };

        if !is_equal {
            println!("Expected:");
            for line in input
                .lines()
                .map(|line| line.trim())
                .filter(|line| !line.is_empty())
            {
                for ch in line.chars() {
                    print!("{ch} ");
                }
                println!();
            }
            println!();
            println!("Got:");
            print_grid(&lines);
            assert!(false);
        }
    }

    fn print_grid(lines: &Vec<Vec<char>>) {
        for line in lines {
            for ch in line {
                print!("{ch} ");
            }
            println!();
        }
        println!();
    }
}
