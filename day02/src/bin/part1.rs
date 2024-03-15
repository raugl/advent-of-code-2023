fn process(input: &str) -> i32 {
    input
        .lines()
        .filter_map(|line| {
            let (index, sets) = line.split_once(':')?;
            let index = index.trim().split_at(5).1.parse::<i32>().ok()?;

            let invalid_count = sets
                .trim()
                .split(';')
                .flat_map(|set| set.trim().split(','))
                .filter_map(|color_counts| {
                    let (count, color) = color_counts.trim().split_once(' ')?;
                    let count = count.parse::<i32>().ok()?;

                    match color {
                        "red" if count > 12 => Some(()),
                        "green" if count > 13 => Some(()),
                        "blue" if count > 14 => Some(()),
                        _ => None,
                    }
                })
                .count();

            match invalid_count {
                0 => Some(index),
                _ => None,
            }
        })
        .sum()
}

fn main() {
    let input = include_str!("./input.txt");
    println!("result: {}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        ";
        assert_eq!(process(input), 8);
    }
}
