fn process(input: &str) -> i64 {
    let instructions = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .flat_map(|line| line.split(','));

    let mut map = HashMap::new();
    for instruction in instructions {
        if instruction.contains('-') {
            let key = instruction.trim_end_matches('-');
            map.remove(key);
        } else {
            let mut tokens = instruction.split('=');
            let key = tokens.next().unwrap();
            let val = tokens.next().unwrap().parse().unwrap();
            map.insert(key, val);
        }
    }

    let mut sum = 0;
    for (i, bucket) in map.buckets.iter().enumerate() {
        let box_num = (i + 1) as i64;
        for (i, pair) in bucket.iter().enumerate() {
            let slot_num = (i + 1) as i64;
            sum += box_num * slot_num * pair.val as i64;
        }
    }
    sum
}

type Bucket<'a> = Vec<KeyVal<'a>>;

#[derive(Default, Debug, Clone)]
struct KeyVal<'a> {
    key: &'a str,
    val: i8,
}

struct HashMap<'a> {
    buckets: [Bucket<'a>; 256],
    hash: Box<dyn Fn(&str) -> u8>,
}

impl<'a> HashMap<'a> {
    fn new() -> Self {
        let hash = |str: &str| -> u8 {
            str.chars()
                .fold(0, |acc, ch| acc.wrapping_add(ch as u8).wrapping_mul(17))
        };

        const ARRAY_REPEAT_VALUE: Vec<KeyVal> = Vec::new();
        Self {
            hash: Box::new(hash),
            buckets: [ARRAY_REPEAT_VALUE; 256],
        }
    }

    fn insert(&mut self, key: &'a str, val: i8) -> Option<i8> {
        let hash = (self.hash)(key) as usize;
        let bucket = &mut self.buckets[hash];

        if let Some(pair) = bucket.iter_mut().find(|pair| pair.key == key) {
            let old_val = pair.val;
            pair.val = val;
            Some(old_val)
        } else {
            bucket.push(KeyVal { key, val });
            None
        }
    }

    fn remove(&mut self, key: &str) -> Option<i8> {
        let hash = (self.hash)(key) as usize;
        let bucket = &mut self.buckets[hash];

        if let Some(idx) = bucket
            .iter()
            .enumerate()
            .find(|(_i, pair)| pair.key == key)
            .map(|(i, _pair)| i)
        {
            Some(bucket.remove(idx).val)
        } else {
            None
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(process(input), 145);
    }
}

