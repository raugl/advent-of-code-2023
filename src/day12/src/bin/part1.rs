fn process(input: &str) -> i64 {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| parse_line(line))
        .map(|(row, lens)| dfs(row, &lens, String::new()))
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

fn eq_perm(perm: &String, lens: &[usize]) -> bool {
    let perm_lens = perm
        .split('.')
        .filter(|split| !split.is_empty())
        .map(|seq| seq.len());

    lens.iter().map(|len| *len).eq(perm_lens)
}

fn dfs(row: &str, lens: &[usize], perm: String) -> i64 {
    if row.is_empty() {
        return eq_perm(&perm, lens) as i64;
    }

    let helper = |ch, mut perm: String| {
        perm.push(ch);
        dfs(&row[1..], lens, perm)
    };

    match row.chars().next().unwrap() {
        '.' => helper('.', perm),
        '#' => helper('#', perm),
        '?' => helper('#', perm.clone()) + helper('.', perm),
        _ => unreachable!(),
    }
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
            ???.### 1,1,3
            .??..??...?##. 1,1,3
            ?#?#?#?#?#?#?#? 1,3,1,6
            ????.#...#... 4,1,1
            ????.######..#####. 1,6,5
            ?###???????? 3,2,1
        ";
        assert_eq!(process(input), 21);
    }
}
