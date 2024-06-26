use std::collections::{BTreeMap, VecDeque};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy, Debug)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn new(pulse: bool) -> Self {
        match pulse {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
struct Message<'a> {
    pulse: Pulse,
    src: &'a str,
    dest: &'a str,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default, Debug)]
struct Broadcaster<'a> {
    outputs: Vec<&'a str>,
}

impl<'a> Broadcaster<'a> {
    fn update(&self, self_name: &'a str, msg: Message<'a>) -> Vec<Message<'a>> {
        self.outputs
            .iter()
            .map(|output| Message {
                pulse: msg.pulse,
                src: self_name,
                dest: *output,
            })
            .collect()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default, Debug)]
struct FlipFlop<'a> {
    state: bool,
    outputs: Vec<&'a str>,
}

impl<'a> FlipFlop<'a> {
    fn update(&mut self, self_name: &'a str, msg: Message<'a>) -> Vec<Message<'a>> {
        match msg.pulse {
            Pulse::Low => {
                self.state = !self.state;
                self.outputs
                    .iter()
                    .map(|output| Message {
                        pulse: Pulse::new(self.state),
                        src: self_name,
                        dest: *output,
                    })
                    .collect()
            }
            Pulse::High => vec![],
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default, Debug)]
struct Conjunction<'a> {
    inputs: BTreeMap<&'a str, Pulse>,
    outputs: Vec<&'a str>,
}

impl<'a> Conjunction<'a> {
    fn update(&mut self, self_name: &'a str, msg: Message<'a>) -> Vec<Message<'a>> {
        *self.inputs.get_mut(msg.src).unwrap() = msg.pulse;

        let pulse = match self.inputs.values().all(|input| *input == Pulse::High) {
            true => Pulse::Low,
            false => Pulse::High,
        };

        self.outputs
            .iter()
            .map(|output| Message {
                pulse,
                src: self_name,
                dest: *output,
            })
            .collect()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Debug)]
enum Module<'a> {
    FlipFlop(FlipFlop<'a>),
    Conjunction(Conjunction<'a>),
    Broadcaster(Broadcaster<'a>),
}

fn process(input: &str) -> i64 {
    let mut modules = parse_input(input);
    let mut factors = BTreeMap::new();

    let mut i = 0;
    while factors.len() < 4 {
        i += 1;

        let mut queue = VecDeque::from([Message {
            pulse: Pulse::Low,
            src: "button",
            dest: "broadcaster",
        }]);

        while let Some(msg) = queue.pop_front() {
            let module = match modules.get_mut(msg.dest) {
                Some(module) => module,
                None => continue,
            };

            let messages = match module {
                Module::FlipFlop(module) => module.update(msg.dest, msg),
                Module::Conjunction(module) => module.update(msg.dest, msg),
                Module::Broadcaster(module) => module.update(msg.dest, msg),
            };

            for msg in messages.iter() {
                if msg.pulse == Pulse::Low
                    && ["kv", "jg", "rz", "mr"].contains(&msg.dest)
                    && factors.get(msg.dest).is_none()
                {
                    factors.insert(msg.dest, i);
                }
            }
            queue.extend(messages);
        }
    }
    factors.values().product()
}

fn parse_input<'a>(input: &'a str) -> BTreeMap<&str, Module<'a>> {
    let lines = input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty());

    let mut modules = BTreeMap::new();
    for line in lines {
        let (module, outputs) = line.split_once(" -> ").unwrap();
        let outputs: Vec<&str> = outputs.split(", ").collect();

        match module.chars().nth(0).unwrap() {
            '%' => {
                let name = &module[1..];
                let module = Module::FlipFlop(FlipFlop {
                    state: false,
                    outputs,
                });
                modules.insert(name, module);
            }
            '&' => {
                let name = &module[1..];
                let module = Module::Conjunction(Conjunction {
                    inputs: BTreeMap::new(),
                    outputs,
                });
                modules.insert(name, module);
            }
            _ => {
                let name = module; // "broadcaster"
                let module = Module::Broadcaster(Broadcaster { outputs });
                modules.insert(name, module);
            }
        }
    }

    // PERF: I'm forced by the borrow checker to clone the map. This could maybe be fixed by adding a
    // bunch of RefCells, but that sounds really annoying.
    for (name, module) in modules.clone().iter() {
        let outputs = match module {
            Module::FlipFlop(module) => &module.outputs,
            Module::Conjunction(module) => &module.outputs,
            Module::Broadcaster(module) => &module.outputs,
        };

        for output in outputs {
            match modules.get_mut(output) {
                Some(Module::Conjunction(conjunction)) => {
                    conjunction.inputs.insert(name, Pulse::Low);
                }
                _ => (),
            }
        }
    }
    modules
}

fn main() {
    let input = include_str!("input.txt");
    println!("Result: {}", process(input));
}
