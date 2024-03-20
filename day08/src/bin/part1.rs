use std::collections::HashMap;

fn process(input: &str) -> i64 {
    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let steps = lines.next().unwrap().chars().cycle();
    let map = parse_map(lines);

    let (mut count, mut node) = (0, "AAA");
    for step in steps {
        if node == "ZZZ" {
            return count;
        }

        count += 1;
        node = if step == 'L' {
            map.get(&node).unwrap().0
        } else if step == 'R' {
            map.get(&node).unwrap().1
        } else {
            panic!("Unexpected step direction: {step}")
        };
    }

    unreachable!()
}

fn parse_map<'a>(lines: impl Iterator<Item = &'a str>) -> HashMap<&'a str, (&'a str, &'a str)> {
    lines
        .map(|line| {
            let mut tokens = line.split('=');
            let key = tokens.next().unwrap().trim();
            let mut tokens = tokens
                .next()
                .unwrap()
                .trim()
                .trim_matches(&['(', ')'])
                .split(',');

            let left = tokens.next().unwrap().trim();
            let right = tokens.next().unwrap().trim();

            (key, (left, right))
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
            RL

            AAA = (BBB, CCC)
            BBB = (DDD, EEE)
            CCC = (ZZZ, GGG)
            DDD = (DDD, DDD)
            EEE = (EEE, EEE)
            GGG = (GGG, GGG)
            ZZZ = (ZZZ, ZZZ)
        ";
        assert_eq!(process(input), 2);

        let input = "
            LLR

            AAA = (BBB, BBB)
            BBB = (AAA, ZZZ)
            ZZZ = (ZZZ, ZZZ)
        ";
        assert_eq!(process(input), 6);
    }
}
