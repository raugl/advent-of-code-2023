fn process(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.trim().as_bytes())
        .filter(|line| !line.is_empty());

    let galaxies = get_galaxies(lines.clone());
    let presums = get_presums(lines);

    let mut sum = 0;
    for (i, galaxy1) in galaxies.iter().enumerate() {
        for (j, galaxy2) in galaxies.iter().enumerate() {
            if i <= j {
                break;
            }
            sum += dist(galaxy1, galaxy2, &presums);
        }
    }
    sum
}

fn get_galaxies<'a>(lines: impl Iterator<Item = &'a [u8]>) -> Vec<(usize, usize)> {
    lines
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, ch)| ((x, y), ch)))
        .filter(|(_pos, ch)| **ch == b'#')
        .map(|(pos, _ch)| pos)
        .collect()
}

fn get_presums<'a, Iter>(lines: Iter) -> (Vec<i64>, Vec<i64>)
where
    Iter: Iterator<Item = &'a [u8]> + Clone,
{
    let mut presum_y = Vec::new();
    for line in lines.clone() {
        let is_empty = line.iter().all(|ch| *ch == b'.') as i64;
        presum_y.push(presum_y.last().unwrap_or(&0) + 1 + is_empty);
    }

    let mut presum_x = Vec::new();
    let width = lines.clone().next().unwrap().len();

    for x in 0..width {
        let is_empty = lines.clone().map(|line| line[x]).all(|ch| ch == b'.') as i64;
        presum_x.push(presum_x.last().unwrap_or(&0) + 1 + is_empty);
    }

    (presum_x, presum_y)
}

fn dist(start: &(usize, usize), end: &(usize, usize), presums: &(Vec<i64>, Vec<i64>)) -> i64 {
    let dist_x = (presums.0[end.0] - presums.0[start.0]).abs();
    let dist_y = (presums.1[end.1] - presums.1[start.1]).abs();
    dist_x + dist_y
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
            ...#......
            .......#..
            #.........
            ..........
            ......#...
            .#........
            .........#
            ..........
            .......#..
            #...#.....
        ";
        assert_eq!(process(input), 374);
    }
}
