fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(',').map(|instruction| hash(instruction) as i64))
        .sum()
}

fn hash(input: &str) -> u8 {
    input
        .chars()
        .fold(0, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process(input), 1320);
    }
}
