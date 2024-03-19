use std::cmp;

fn process(input: &str) -> i64 {
    let mut lines = input.lines().filter_map(|line| {
        let line = line.trim();
        match line.is_empty() {
            false => Some(line),
            true => None,
        }
    });

    let vals = lines
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .flat_map(|str| str.trim().split(' '))
        .map(|num| num.parse().unwrap())
        .collect::<Vec<i64>>();

    let mut vals = (vals.iter().step_by(2))
        .zip(vals.iter().skip(1).step_by(2))
        .map(|(start, len)| Range::new(*start, *len))
        .collect::<Vec<_>>();

    let maps = parse_maps(lines);
    for map in maps {
        vals = vals.into_iter().map(|val| map.map(val)).flatten().collect();
    }

    vals.into_iter().map(|range| range.start).min().unwrap()
}

fn parse_maps<'a>(lines: impl Iterator<Item = &'a str>) -> Vec<Map> {
    let mut maps = Vec::new();

    for line in lines {
        let is_header = !(line.as_bytes()[0] as char).is_ascii_digit();
        if is_header {
            maps.push(Map::default());
            continue;
        }

        let nums = line
            .split(' ')
            .map(|num| num.parse::<i64>().unwrap())
            .collect::<Vec<_>>();

        maps.last_mut()
            .expect("I just pushed an item, you are missing a section header in the input")
            .mappers
            .push(RangeMapper::new(nums[0], nums[1], nums[2]));
    }
    maps
}

#[derive(Debug, Clone)]
struct Range {
    start: i64,
    len: i64,
}

impl Range {
    fn new(start: i64, len: i64) -> Self {
        Self { start, len }
    }

    fn checked(start: i64, len: i64) -> Option<Self> {
        if len > 0 {
            Some(Self { start, len })
        } else {
            None
        }
    }

    fn intersect(&self, other: &Range) -> Option<Range> {
        let start = cmp::max(self.start, other.start);
        let end = cmp::min(self.start + self.len, other.start + other.len);
        if start >= end {
            return None;
        }

        Some(Range::new(start, end - start))
    }
}

struct RangeMapper {
    src_range: Range,
    dest_start: i64,
}

impl RangeMapper {
    fn new(dest_start: i64, src_start: i64, len: i64) -> Self {
        Self {
            src_range: Range::new(src_start, len),
            dest_start,
        }
    }

    fn map(&self, range: &Range) -> Option<(Range, [Option<Range>; 2])> {
        let src_end = self.src_range.start + self.src_range.len;
        let range_end = range.start + range.len;

        if let Some(intersection) = self.src_range.intersect(range) {
            let pre_diff = Range::checked(range.start, self.src_range.start - range.start);
            let post_diff = Range::checked(src_end, range_end - src_end);

            let mapped_range = Range::new(
                self.dest_start + (intersection.start - self.src_range.start),
                intersection.len,
            );
            return Some((mapped_range, [pre_diff, post_diff]));
        }
        None
    }
}

#[derive(Default)]
struct Map {
    pub mappers: Vec<RangeMapper>,
}

impl Map {
    fn map(&self, range: Range) -> Vec<Range> {
        let mut src_ranges = vec![range];
        let mut mapped_ranges = Vec::new();

        for mapper in self.mappers.iter() {
            src_ranges = src_ranges
                .into_iter()
                .flat_map(|range| {
                    if let Some((mapped_range, diffs)) = mapper.map(&range) {
                        mapped_ranges.push(mapped_range);
                        return diffs;
                    }
                    [Some(range), None]
                })
                .flatten()
                .collect();
        }

        mapped_ranges.extend(src_ranges.into_iter());
        mapped_ranges
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("result: {}", process(input));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_process() {
        let input = "
            seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4
        ";
        assert_eq!(process(input), 46);
    }
}
