fn process(input: &str) -> i64 {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut tokens = line.split(' ');
            let dir = tokens.next().unwrap();
            let len: i64 = tokens.next().unwrap().parse().unwrap();
            (dir, len)
        });

    let vertices: Vec<(i64, i64)> = lines
        .clone()
        .scan((0, 0), |pos, (dir, len)| {
            match dir {
                "U" => pos.1 -= len,
                "D" => pos.1 += len,
                "L" => pos.0 -= len,
                "R" => pos.0 += len,
                _ => unreachable!(),
            }
            Some(pos.clone())
        })
        .collect();

    // Shoelace area formula and Pick's theorem
    let perimeter: i64 = lines.map(|(_dir, len)| len).sum();
    let area: i64 = vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(pos, next)| (pos.1 + next.1) * (pos.0 - next.0))
        .sum();

    (area + perimeter) / 2 + 1
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
            R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)
        ";
        assert_eq!(process(input), 62);
    }
}

