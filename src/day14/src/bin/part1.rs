use std::{cell::RefCell, mem};

fn process(input: &str) -> i64 {
    let lines: Vec<_> = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| RefCell::new(line.chars().collect::<Vec<char>>()))
        .collect();

    loop {
        let mut moved_count = 0;
        for (prev, line) in lines.iter().zip(lines.iter().skip(1)) {
            prev.borrow_mut()
                .iter_mut()
                .zip(line.borrow_mut().iter_mut())
                .filter(|(prev, ch)| **prev == '.' && **ch == 'O')
                .for_each(|(prev, ch)| {
                    mem::swap(prev, ch);
                    moved_count += 1
                });
        }

        if moved_count == 0 {
            break;
        }
    }

    lines
        .iter()
        .rev()
        .enumerate()
        .map(|(i, line)| line.borrow().iter().filter(|ch| **ch == 'O').count() * (i + 1))
        .sum::<usize>() as i64
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
        assert_eq!(process(input), 136);
    }
}
