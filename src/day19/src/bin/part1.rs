use std::{collections::HashMap, iter};

fn process(input: &str) -> i64 {
    let mut lines = trim_leading(input).lines().map(|line| line.trim());
    let workflows = parse_workflows(&mut lines);
    let parts = parse_parts(&mut lines);

    parts
        .into_iter()
        .map(|part| {
            let last = iter::successors(Some("in"), |workflow_name| {
                workflows
                    .get(workflow_name)
                    .unwrap()
                    .iter()
                    .find_map(|rule| rule.next_workflow(&part))
            })
            .last();

            match last {
                Some("R") => 0,
                Some("A") => (part.x + part.m + part.a + part.s) as i64,
                _ => unreachable!(),
            }
        })
        .sum()
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

fn parse_workflows<'a>(
    input: &mut impl Iterator<Item = &'a str>,
) -> HashMap<&'a str, Vec<Rule<'a>>> {
    let mut map = HashMap::from([("A", Vec::new()), ("R", Vec::new())]);

    for line in input {
        if line.is_empty() {
            break;
        }

        let (name, rules) = line.split_once('{').unwrap();
        let rules: Vec<_> = rules[..rules.len() - 1]
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

fn parse_parts<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<Part> {
    let mut parts = Vec::new();

    for line in input {
        if line.is_empty() {
            break;
        }

        let mut part = Part::default();
        for attrib in line[1..line.len() - 1].split(',') {
            let (attrib, value) = attrib.split_once('=').unwrap();
            let value: i32 = value.parse().unwrap();

            match attrib {
                "x" => part.x = value,
                "m" => part.m = value,
                "a" => part.a = value,
                "s" => part.s = value,
                _ => unreachable!(),
            }
        }

        parts.push(part);
    }
    parts
}

#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Part {
    x: i32,
    m: i32,
    a: i32,
    s: i32,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
enum Condition {
    XLess(i32),
    XGrater(i32),
    MLess(i32),
    MGrater(i32),
    ALess(i32),
    AGrater(i32),
    SLess(i32),
    SGrater(i32),
}

impl Condition {
    fn parse(cond: &str) -> Condition {
        let attrib = cond.as_bytes()[0] as char;
        let operator = cond.as_bytes()[1] as char;
        let value: i32 = cond[2..].parse().unwrap();
        use Condition::*;

        match attrib {
            'x' => match operator {
                '>' => XGrater(value),
                '<' => XLess(value),
                _ => unreachable!(),
            },
            'm' => match operator {
                '>' => MGrater(value),
                '<' => MLess(value),
                _ => unreachable!(),
            },
            'a' => match operator {
                '>' => AGrater(value),
                '<' => ALess(value),
                _ => unreachable!(),
            },
            's' => match operator {
                '>' => SGrater(value),
                '<' => SLess(value),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Default, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Rule<'a> {
    cond: Option<Condition>,
    next: &'a str,
}

impl<'a> Rule<'a> {
    fn next_workflow(&self, part: &Part) -> Option<&'a str> {
        use Condition::*;

        match self.cond {
            None => Some(self.next),
            Some(XLess(val)) => (part.x < val).then_some(self.next),
            Some(MLess(val)) => (part.m < val).then_some(self.next),
            Some(ALess(val)) => (part.a < val).then_some(self.next),
            Some(SLess(val)) => (part.s < val).then_some(self.next),
            Some(XGrater(val)) => (part.x > val).then_some(self.next),
            Some(MGrater(val)) => (part.m > val).then_some(self.next),
            Some(AGrater(val)) => (part.a > val).then_some(self.next),
            Some(SGrater(val)) => (part.s > val).then_some(self.next),
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
        assert_eq!(process(input), 19114);
    }
}
