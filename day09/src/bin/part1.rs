use std::mem;

fn process(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    lines
        .map(|line| line.split(' ').map(|num| num.parse().unwrap()))
        .map(|line| extrapolate(line))
        .sum()
}

fn extrapolate(nums: impl Iterator<Item = i64>) -> i64 {
    let mut tail_diffs = Vec::new();
    let mut nums = nums.collect::<Vec<_>>();

    while !nums.iter().all(|num| *num == 0) {
        let mut last = nums.pop().unwrap();
        tail_diffs.push(last);

        for num in nums.iter_mut().rev() {
            *num = mem::replace(&mut last, *num) - *num;
        }
    }
    tail_diffs.into_iter().sum()
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
            0 3 6 9 12 15
            1 3 6 10 15 21
            10 13 16 21 30 45
        ";
        assert_eq!(process(input), 114);
    }
}
