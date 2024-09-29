use std::collections::{BTreeMap, VecDeque};

use aoc_core::{AocResult, Day};
use itertools::Itertools;

pub fn day() -> Day {
    let mut solution = Day::new(20);
    solution.part_1(parse, part_one);
    // solution.part_2(parse, part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: String) -> (BTreeMap<String, Module>, Vec<String>) {
    let mut modules = BTreeMap::new();
    let mut broadcaster_output = None;

    for line in input.split_terminator('\n') {
        let (name_raw, output_raw) = line.split_once(" -> ").unwrap();

        if name_raw == "broadcaster" {
            broadcaster_output = Some(output_raw.split(", ").map(str::to_owned).collect());
            continue;
        }

        let kind = match &name_raw[0..1] {
            "%" => ModuleKind::FlipFlop(Signal::Low),
            "&" => ModuleKind::Conjunction(BTreeMap::new()),
            _ => unreachable!(),
        };

        modules.insert(
            name_raw[1..].to_owned(),
            Module::new(kind, output_raw.split(", ").map(str::to_owned).collect()),
        );
    }

    for module_name in modules.keys().cloned().collect_vec() {
        for out in modules[&module_name].output.clone() {
            if let Some(ModuleKind::Conjunction(map)) =
                &mut modules.get_mut(&out).map(|module| &mut module.kind)
            {
                map.insert(module_name.to_owned(), Signal::Low);
            }
        }
    }

    (modules, broadcaster_output.unwrap())
}

fn part_one(
    (mut modules, broadcaster_output): (BTreeMap<String, Module>, Vec<String>),
) -> AocResult<u32> {
    let mut high_signals = 0;
    let mut low_signals = 0;

    for _ in 0..1000 {
        low_signals += 1;

        let mut queue = VecDeque::new();

        for out in &broadcaster_output {
            queue.push_back(("broadcaster".to_owned(), out.to_owned(), Signal::Low));
        }

        while let Some((from, to, signal)) = queue.pop_front() {
            match signal {
                Signal::High => high_signals += 1,
                Signal::Low => low_signals += 1,
            }

            let Some(module) = modules.get_mut(&to) else {
                continue;
            };

            match &mut module.kind {
                ModuleKind::FlipFlop(current) => {
                    if let Signal::High = signal {
                        continue;
                    }

                    match current {
                        Signal::High => *current = Signal::Low,
                        Signal::Low => *current = Signal::High,
                    }

                    for out in &module.output {
                        queue.push_back((to.clone(), out.to_owned(), *current));
                    }
                }
                ModuleKind::Conjunction(history) => {
                    *history.get_mut(&from).unwrap() = signal;

                    let send = if history.values().all(|signal| *signal == Signal::High) {
                        Signal::Low
                    } else {
                        Signal::High
                    };

                    for out in &module.output {
                        queue.push_back((to.clone(), out.to_owned(), send));
                    }
                }
            }
        }
    }

    Ok(high_signals * low_signals)
}

// fn part_two(
//     (mut modules, broadcaster_output): (BTreeMap<String, Module>, Vec<String>),
// ) -> AocResult<u32> {
//     let mut presses = 0;
//     let mut queue = VecDeque::new();

//     loop {
//         presses += 1;

//         for out in &broadcaster_output {
//             queue.push_back(("broadcaster".to_owned(), out.to_owned(), Signal::Low));
//         }

//         while let Some((from, to, signal)) = queue.pop_front() {
//             if to.eq("rx") && signal == Signal::Low {
//                 return Ok(presses);
//             }

//             let Some(module) = modules.get_mut(&to) else {
//                 continue;
//             };

//             match &mut module.kind {
//                 ModuleKind::FlipFlop(current) => {
//                     if let Signal::High = signal {
//                         continue;
//                     }

//                     match current {
//                         Signal::High => *current = Signal::Low,
//                         Signal::Low => *current = Signal::High,
//                     }

//                     for out in &module.output {
//                         queue.push_back((to.clone(), out.to_owned(), *current));
//                     }
//                 }
//                 ModuleKind::Conjunction(history) => {
//                     *history.get_mut(&from).unwrap() = signal;

//                     let send = if history.values().all(|signal| *signal == Signal::High) {
//                         Signal::Low
//                     } else {
//                         Signal::High
//                     };

//                     for out in &module.output {
//                         queue.push_back((to.clone(), out.to_owned(), send));
//                     }
//                 }
//             }
//         }
//     }
// }

#[derive(Clone)]
struct Module {
    kind: ModuleKind,
    output: Vec<String>,
}

impl Module {
    fn new(kind: ModuleKind, output: Vec<String>) -> Self {
        Self { kind, output }
    }
}

#[derive(Clone)]
enum ModuleKind {
    FlipFlop(Signal),
    Conjunction(BTreeMap<String, Signal>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Signal {
    High,
    Low,
}
