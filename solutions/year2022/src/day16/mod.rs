use std::collections::{HashMap, HashSet};

use aoc_core::{Day, YearDay};
use regex::Regex;

pub fn day() -> Day {
    let mut solution = Day::new(YearDay::Day16);
    solution.part_1(parse, part_one);
    solution.add_file("input.txt");
    solution
}

fn part_one(valves: HashMap<String, Valve>) -> String {
    let mut valves = valves.clone();

    let keys = valves.keys().map(String::clone).collect::<Vec<_>>();

    let mut distancess = HashMap::new();

    for key in keys {
        let mut distances = HashMap::new();
        find_distances(&key, &valves, &mut distances, 0);

        distances.remove(&key);
        distancess.insert(key, distances);
    }

    for (valve_name, distances) in distancess {
        valves.get_mut(&valve_name).unwrap().distances = distances;
    }

    let keys = valves.keys().map(String::clone).collect::<Vec<_>>();
    for key in keys {
        if valves[&key].rate == 0 {
            if key != "AA" {
                valves.remove(&key);
            }

            for (_, valve) in &mut valves {
                valve.distances.remove(&key);
            }

            continue;
        }
    }

    let highest = start_search1(&valves);

    highest.to_string()
}

fn parse(input: String) -> HashMap<String, Valve> {
    let regex = Regex::new(r"^Valve (?P<name>[A-Z]{2}) has flow rate=(?P<rate>\d+); tunnels? leads? to valves? (?P<distances>[A-Z]{2}(, [A-Z]{2})*)$").unwrap();

    let mut valves = HashMap::new();

    for line in input.split_terminator("\n") {
        let captures = regex.captures(line).unwrap();

        let name = captures["name"].to_owned();
        let rate = captures["rate"].parse().unwrap();
        let mut distances = HashMap::new();
        for valve_name in captures["distances"].split(", ") {
            distances.insert(valve_name.to_owned(), 1);
        }

        let valve = Valve::new(name.clone(), rate, distances);
        valves.insert(name, valve);
    }

    valves
}

fn find_distances(
    valve_name: &String,
    valves: &HashMap<String, Valve>,
    distances: &mut HashMap<String, u32>,
    distance: u32,
) {
    for (distance_name, _) in &valves[valve_name].distances {
        if distances.contains_key(distance_name) {
            if distance + 1 < distances[distance_name] {
                distances.insert(distance_name.clone(), distance + 1);
                find_distances(distance_name, valves, distances, distance + 1);
            }

            continue;
        }

        distances.insert(distance_name.clone(), distance + 1);
        find_distances(distance_name, valves, distances, distance + 1);
    }
}

fn start_search1(valves: &HashMap<String, Valve>) -> u32 {
    let mut highest = 0;

    for (distance_name, distance) in &valves["AA"].distances {
        let value = search1(distance_name, &valves, &mut HashSet::new(), *distance);
        if value > highest {
            highest = value;
        }
    }

    highest
}

fn search1(
    valve_name: &String,
    valves: &HashMap<String, Valve>,
    opened: &mut HashSet<String>,
    time: u32,
) -> u32 {
    let mut highest = 0;

    opened.insert(valve_name.clone());

    for (distance_name, distance) in &valves[valve_name].distances {
        if time + distance < 29 && !opened.contains(distance_name) {
            let value = search1(distance_name, valves, opened, time + distance + 1);
            if value > highest {
                highest = value;
            }
        }
    }

    opened.remove(valve_name);

    highest += valves[valve_name].rate * (30 - time - 1);

    highest
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Valve {
    name: String,
    rate: u32,
    distances: HashMap<String, u32>,
}

impl Valve {
    fn new(name: String, rate: u32, distances: HashMap<String, u32>) -> Self {
        Self {
            name,
            rate,
            distances,
        }
    }
}
