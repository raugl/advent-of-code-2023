use std::collections::HashMap;

fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (row, lens) = parse_line(line);
            let (row_len, lens_len) = (row.len() * 5 + 4, lens.len() * 5);

            let lens: Vec<_> = lens.into_iter().cycle().take(lens_len).collect();
            let row: String = row
                .chars()
                .chain(['?'].into_iter())
                .cycle()
                .take(row_len)
                .collect();

            let mut cache = HashMap::new();
            return dfs(0, 0, &row, Some(&lens), &mut cache);
        })
        .sum()
}

fn parse_line(line: &str) -> (&str, Vec<usize>) {
    let mut tokens = line.split(' ');
    let row = tokens.next().unwrap();
    let lens = tokens
        .next()
        .unwrap()
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();

    (row, lens)
}

fn dfs(
    start: usize,
    len: usize,
    row: &str,
    lens: Option<&[usize]>,
    cache: &mut HashMap<(usize, usize, usize), i64>,
) -> i64 {
    let target = *lens.and_then(|lens| lens.first()).unwrap_or(&0);
    let lens_len = lens.map_or(0, |lens| lens.len());
    let key = (start, len, lens_len);

    if let Some(val) = cache.get(&key) {
        return *val;
    }

    if row.is_empty() {
        let trailing_dots = len == 0 && lens_len == 0;
        let last_match = len == target && lens_len == 1;
        return (trailing_dots || last_match) as i64;
    }

    let hash = |cache| dfs(start + 1, len + 1, &row[1..], lens, cache);
    let dot = |cache| match len {
        len if len == target => {
            let lens = lens.and_then(|lens| lens.get(1..));
            dfs(start + 1, 0, &row[1..], lens, cache)
        }
        0 => dfs(start + 1, 0, &row[1..], lens, cache),
        _ => 0,
    };

    let result = match row.chars().next().unwrap() {
        '.' => dot(cache),
        '#' => hash(cache),
        '?' => dot(cache) + hash(cache),
        _ => unreachable!(),
    };

    cache.insert(key, result);
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
        assert_eq!(process("???.### 1,1,3"), 1);
        assert_eq!(process(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(process("?#?#?#?#?#?#?#? 1,3,1,6"), 1);
        assert_eq!(process("????.#...#... 4,1,1"), 16);
        assert_eq!(process("????.######..#####. 1,6,5"), 2500);
        assert_eq!(process("?###???????? 3,2,1"), 506250);
    }
}
