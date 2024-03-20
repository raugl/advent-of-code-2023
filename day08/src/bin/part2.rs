use std::collections::{BTreeMap, HashMap};

fn process(input: &str) -> i64 {
    let mut lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let steps = lines.next().unwrap().chars().cycle();
    let map = parse_map(lines);

    least_common_multiple(
        map.keys()
            .filter(|key| key.chars().last().unwrap() == 'A')
            .map(|key| steps_to_end(key, &map, steps.clone())),
    )
}

type MapType<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn parse_map<'a>(lines: impl Iterator<Item = &'a str>) -> MapType<'a> {
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

fn steps_to_end(start: &str, map: &MapType, steps: impl Iterator<Item = char>) -> i64 {
    let (mut count, mut node) = (0, start);
    for step in steps {
        if node.chars().last().unwrap() == 'Z' {
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

fn least_common_multiple(items: impl Iterator<Item = i64>) -> i64 {
    let mut prime_counts = BTreeMap::new();

    for num in items {
        let factors = factorize(num);
        for (prime, count) in factors {
            prime_counts
                .entry(prime)
                .and_modify(|old_count| *old_count = count.max(*old_count))
                .or_insert(count);
        }
    }

    prime_counts
        .into_iter()
        .map(|(prime, count)| prime * count)
        .product()
}

fn factorize(mut num: i64) -> BTreeMap<i64, i64> {
    let primes = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997,
    ];

    let mut result = BTreeMap::new();
    while num > 1 {
        for prime in primes {
            if num % prime == 0 {
                num /= prime;
                result
                    .entry(prime)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
            }
        }
    }
    result
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
            LR

            11A = (11B, XXX)
            11B = (XXX, 11Z)
            11Z = (11B, XXX)
            22A = (22B, XXX)
            22B = (22C, 22C)
            22C = (22Z, 22Z)
            22Z = (22B, 22B)
            XXX = (XXX, XXX)
        ";
        assert_eq!(process(input), 6);
    }
}

