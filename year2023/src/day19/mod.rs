use std::{collections::BTreeMap, ops::Range};

use aoc_core::{AocResult, Day};

pub fn day() -> Day {
    let mut solution = Day::new(19);
    solution.part_1(part_one);
    solution.part_2(part_two);
    solution.add_file("files/test.in");
    solution.add_file("files/input.in");
    solution
}

fn parse(input: &String) -> (BTreeMap<String, Workflow>, Vec<Part>) {
    let (workflow_lines, part_lines) = input.split_once("\n\n").unwrap();

    let workflows = workflow_lines
        .split_terminator('\n')
        .map(|line| {
            let (name, rules_raw) = line.split_once('{').unwrap();

            let mut rules = Vec::new();

            for rule in rules_raw[..rules_raw.len() - 1].split(',') {
                if let Some((condition, workflow)) = rule.split_once(':') {
                    let variable = match &condition[0..1] {
                        "x" => Variable::X,
                        "m" => Variable::M,
                        "a" => Variable::A,
                        "s" => Variable::S,
                        _ => unreachable!(),
                    };

                    let operator = match &condition[1..2] {
                        ">" => Operator::Gt,
                        "<" => Operator::Lt,
                        _ => unreachable!(),
                    };

                    let value = condition[2..].parse().unwrap();

                    rules.push((Rule::new(variable, operator, value), workflow.to_owned()));
                } else {
                    return (name.to_owned(), Workflow::new(rules, rule.to_owned()));
                }
            }

            unreachable!()
        })
        .collect();

    let parts = part_lines
        .split_terminator('\n')
        .map(|line| {
            let mut split = line[1..line.len() - 1].split(',');

            let x = split.next().unwrap()[2..].parse().unwrap();
            let m = split.next().unwrap()[2..].parse().unwrap();
            let a = split.next().unwrap()[2..].parse().unwrap();
            let s = split.next().unwrap()[2..].parse().unwrap();

            Part::new(x, m, a, s)
        })
        .collect();

    (workflows, parts)
}

fn part_one(input: &String) -> AocResult<u64> {
    let (workflows, parts) = parse(input);

    let mut accepted = Vec::new();

    'parts: for part in parts.into_iter() {
        let mut workflow = &workflows["in"];

        'outer: loop {
            for (rule, workflow_name) in &workflow.rules {
                if !rule.matches(&part) {
                    continue;
                }

                match workflow_name.as_str() {
                    "A" => break 'outer,
                    "R" => continue 'parts,
                    _ => {
                        workflow = &workflows[workflow_name];
                        continue 'outer;
                    }
                }
            }

            match workflow.last_rule.as_str() {
                "A" => break,
                "R" => continue 'parts,
                _ => {
                    workflow = &workflows[&workflow.last_rule];
                }
            }
        }

        accepted.push(part);
    }

    Ok(accepted.into_iter().map(|p| p.x + p.m + p.a + p.s).sum())
}

fn part_two(input: &String) -> AocResult<u64> {
    let (workflows, _) = parse(input);

    let part_range = PartRange::new(1..4001, 1..4001, 1..4001, 1..4001);
    Ok(get_accepted(&workflows, &workflows["in"], part_range))
}

fn get_accepted(
    workflows: &BTreeMap<String, Workflow>,
    workflow: &Workflow,
    mut parts: PartRange,
) -> u64 {
    let mut accepted = 0;

    for (rule, workflow_name) in &workflow.rules {
        let (matches, other) = rule.split(&parts);

        if let Some(matches) = matches {
            match workflow_name.as_str() {
                "A" => accepted += matches.size(),
                "R" => (),
                _ => {
                    accepted += get_accepted(workflows, &workflows[workflow_name], matches);
                }
            }
        }

        if let Some(other) = other {
            parts = other;
        } else {
            return accepted;
        }
    }

    match workflow.last_rule.as_str() {
        "A" => accepted += parts.size(),
        "R" => (),
        _ => accepted += get_accepted(workflows, &workflows[&workflow.last_rule], parts),
    }

    accepted
}

struct Workflow {
    rules: Vec<(Rule, String)>,
    last_rule: String,
}

impl Workflow {
    fn new(rules: Vec<(Rule, String)>, last_rule: String) -> Self {
        Self { rules, last_rule }
    }
}

struct Rule {
    variable: Variable,
    operator: Operator,
    value: u64,
}

impl Rule {
    fn new(variable: Variable, operator: Operator, value: u64) -> Self {
        Self {
            variable,
            operator,
            value,
        }
    }

    fn matches(&self, part: &Part) -> bool {
        let part_value = match self.variable {
            Variable::X => part.x,
            Variable::M => part.m,
            Variable::A => part.a,
            Variable::S => part.s,
        };

        match self.operator {
            Operator::Gt => part_value > self.value,
            Operator::Lt => part_value < self.value,
        }
    }

    fn split(&self, parts: &PartRange) -> (Option<PartRange>, Option<PartRange>) {
        let value_range = match self.variable {
            Variable::X => &parts.x,
            Variable::M => &parts.m,
            Variable::A => &parts.a,
            Variable::S => &parts.s,
        };

        match self.operator {
            Operator::Gt => {
                if value_range.start > self.value {
                    (Some(parts.clone()), None)
                } else if value_range.end - 1 <= self.value {
                    (None, Some(parts.clone()))
                } else {
                    match self.variable {
                        Variable::X => (
                            Some(parts.with_x(self.value + 1..parts.x.end)),
                            Some(parts.with_x(parts.x.start..self.value + 1)),
                        ),
                        Variable::M => (
                            Some(parts.with_m(self.value + 1..parts.m.end)),
                            Some(parts.with_m(parts.m.start..self.value + 1)),
                        ),
                        Variable::A => (
                            Some(parts.with_a(self.value + 1..parts.a.end)),
                            Some(parts.with_a(parts.a.start..self.value + 1)),
                        ),
                        Variable::S => (
                            Some(parts.with_s(self.value + 1..parts.s.end)),
                            Some(parts.with_s(parts.s.start..self.value + 1)),
                        ),
                    }
                }
            }
            Operator::Lt => {
                if value_range.end <= self.value {
                    (Some(parts.clone()), None)
                } else if value_range.start >= self.value {
                    (None, Some(parts.clone()))
                } else {
                    match self.variable {
                        Variable::X => (
                            Some(parts.with_x(parts.x.start..self.value)),
                            Some(parts.with_x(self.value..parts.x.end)),
                        ),
                        Variable::M => (
                            Some(parts.with_m(parts.m.start..self.value)),
                            Some(parts.with_m(self.value..parts.m.end)),
                        ),
                        Variable::A => (
                            Some(parts.with_a(parts.a.start..self.value)),
                            Some(parts.with_a(self.value..parts.a.end)),
                        ),
                        Variable::S => (
                            Some(parts.with_s(parts.s.start..self.value)),
                            Some(parts.with_s(self.value..parts.s.end)),
                        ),
                    }
                }
            }
        }
    }
}

enum Variable {
    X,
    M,
    A,
    S,
}

enum Operator {
    Gt,
    Lt,
}

struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    fn new(x: u64, m: u64, a: u64, s: u64) -> Self {
        Self { x, m, a, s }
    }
}

#[derive(Clone)]
struct PartRange {
    x: Range<u64>,
    m: Range<u64>,
    a: Range<u64>,
    s: Range<u64>,
}

impl PartRange {
    fn new(x: Range<u64>, m: Range<u64>, a: Range<u64>, s: Range<u64>) -> Self {
        Self { x, m, a, s }
    }

    fn size(&self) -> u64 {
        self.x.start.abs_diff(self.x.end)
            * self.m.start.abs_diff(self.m.end)
            * self.a.start.abs_diff(self.a.end)
            * self.s.start.abs_diff(self.s.end)
    }

    fn with_x(&self, x: Range<u64>) -> Self {
        Self {
            x,
            m: self.m.clone(),
            a: self.a.clone(),
            s: self.s.clone(),
        }
    }

    fn with_m(&self, m: Range<u64>) -> Self {
        Self {
            x: self.x.clone(),
            m,
            a: self.a.clone(),
            s: self.s.clone(),
        }
    }

    fn with_a(&self, a: Range<u64>) -> Self {
        Self {
            x: self.x.clone(),
            m: self.m.clone(),
            a,
            s: self.s.clone(),
        }
    }

    fn with_s(&self, s: Range<u64>) -> Self {
        Self {
            x: self.x.clone(),
            m: self.m.clone(),
            a: self.a.clone(),
            s,
        }
    }
}
