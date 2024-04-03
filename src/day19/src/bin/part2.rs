use std::{collections::HashMap, ops};

fn process(input: &str) -> i64 {
    let lines = trim_leading(input).lines().map(|line| line.trim());
    let rules = parse_workflows(lines);

    let mut accepted_count = 0;
    let mut stack = vec![Region {
        axes: [1..4001, 1..4001, 1..4001, 1..4001],
        label: "in",
    }];

    while let Some(mut region) = stack.pop() {
        if region.label == "A" {
            accepted_count += region.volume();
        }

        for rule in rules.get(&region.label).unwrap() {
            let (split, remaining) = rule.split_region(region);
            if let Some(split) = split {
                stack.push(split);
            }
            match remaining {
                Some(remaining) => region = remaining,
                None => break,
            }
        }
    }
    accepted_count
}

fn trim_leading(input: &str) -> &str {
    let mut start = 0;
    for line in input.lines() {
        if line.chars().all(char::is_whitespace) {
            start += line.len() + 1;
        } else {
            break;
        }
    }

    &input[start..]
}

fn parse_workflows<'a>(input: impl Iterator<Item = &'a str>) -> HashMap<&'a str, Vec<Rule<'a>>> {
    let mut map = HashMap::from([("A", Vec::new()), ("R", Vec::new())]);

    for line in input {
        if line.is_empty() {
            break;
        }

        let (name, rules) = line.split_once('{').unwrap();
        let rules: Vec<Rule> = rules[..rules.len() - 1]
            .split(',')
            .map(|rule| {
                if let Some((cond, dest)) = rule.split_once(':') {
                    Rule {
                        cond: Some(Condition::parse(cond)),
                        next: dest,
                    }
                } else {
                    Rule {
                        cond: None,
                        next: rule,
                    }
                }
            })
            .collect();

        map.insert(name, rules);
    }
    map
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Axis {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Comparison {
    Less,
    Greater,
}

#[derive(Debug, Hash, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Condition {
    axis: Axis,
    comp: Comparison,
    value: i32,
}

impl Condition {
    fn parse(cond: &str) -> Condition {
        let axis = match cond.chars().nth(0).unwrap() {
            'x' => Axis::X,
            'm' => Axis::M,
            'a' => Axis::A,
            's' => Axis::S,
            _ => unreachable!(),
        };

        let comp = match cond.chars().nth(1).unwrap() {
            '<' => Comparison::Less,
            '>' => Comparison::Greater,
            _ => unreachable!(),
        };

        let value: i32 = cond[2..].parse().unwrap();
        Condition { axis, comp, value }
    }
}

#[derive(Default, Clone, Debug, Hash, PartialEq, Eq)]
struct Region<'a> {
    axes: [ops::Range<i32>; 4],
    label: &'a str,
}

impl<'a> Region<'a> {
    fn volume(&self) -> i64 {
        self.axes.iter().map(|axis| axis.len() as i64).product()
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Rule<'a> {
    cond: Option<Condition>,
    next: &'a str,
}

impl<'a> Rule<'a> {
    fn split_region(&self, mut region: Region<'a>) -> (Option<Region<'a>>, Option<Region<'a>>) {
        match &self.cond {
            None => {
                region.label = self.next;
                (Some(region), None)
            }
            Some(cond) => {
                let range = region.axes[cond.axis as usize].clone();

                match cond.comp {
                    Comparison::Less => {
                        if cond.value <= range.start {
                            (None, Some(region))
                        } else if cond.value >= range.end {
                            region.label = self.next;
                            (Some(region), None)
                        } else {
                            let mut contained = region.clone();
                            contained.axes[cond.axis as usize] = range.start..cond.value;
                            region.axes[cond.axis as usize] = cond.value..range.end;

                            contained.label = self.next;
                            (Some(contained), Some(region))
                        }
                    }
                    Comparison::Greater => {
                        if cond.value < range.start {
                            region.label = self.next;
                            (Some(region), None)
                        } else if cond.value > range.end + 1 {
                            (None, Some(region))
                        } else {
                            let mut excluded = region.clone();
                            region.axes[cond.axis as usize] = (cond.value + 1)..range.end;
                            excluded.axes[cond.axis as usize] = range.start..(cond.value + 1);

                            region.label = self.next;
                            (Some(region), Some(excluded))
                        }
                    }
                }
            }
        }
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
            px{a<2006:qkq,m>2090:A,rfg}
            pv{a>1716:R,A}
            lnx{m>1548:A,A}
            rfg{s<537:gd,x>2440:R,A}
            qs{s>3448:A,lnx}
            qkq{x<1416:A,crn}
            crn{x>2662:A,R}
            in{s<1351:px,qqz}
            qqz{s>2770:qs,m<1801:hdj,R}
            gd{a>3333:R,R}
            hdj{m>838:A,pv}

            {x=787,m=2655,a=1222,s=2876}
            {x=1679,m=44,a=2067,s=496}
            {x=2036,m=264,a=79,s=2244}
            {x=2461,m=1339,a=466,s=291}
            {x=2127,m=1623,a=2188,s=1013}
        ";
        assert_eq!(process(input), 167409079868000);
    }
}
