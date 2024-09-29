use aoc_core::{AocResult, Day};
use regex::Regex;

pub fn day() -> Day {
    let mut solution = Day::new(15);
    solution.part_1(|s: String| part_one(parse(s)));
    solution.part_2(|s: String| part_two(parse(s)));
    solution.add_file("files/input.in");
    solution
}

fn part_one(sensors: Vec<Sensor>) -> AocResult<i32> {
    const Y: i64 = 2000000;

    let mut min_x = 0;
    let mut max_x = 0;

    for sensor in &sensors {
        let distance_y = (Y - sensor.position.y).abs();
        let radius = sensor.distance();

        if sensor.position.x - radius + distance_y < min_x {
            min_x = sensor.position.x - radius + distance_y;
        }
        if sensor.position.x + radius - distance_y > max_x {
            max_x = sensor.position.x + radius - distance_y;
        }
    }

    let mut count = 0;

    for x in min_x..=max_x {
        let position = Position::new(x, Y);

        if sensors.iter().any(|sensor| {
            sensor.beacon != position && sensor.distance() >= sensor.position.distance(&position)
        }) {
            count += 1;
        }
    }

    Ok(count)
}

fn part_two(sensors: Vec<Sensor>) -> AocResult<i64> {
    const SIZE: i64 = 4000000;

    for y in 0..=SIZE {
        let mut x = 0;
        'inner: while x <= SIZE {
            let position = Position::new(x, y);

            for sensor in &sensors {
                let radius = sensor.distance();
                let distance = sensor.position.distance(&position);

                if distance <= radius {
                    x = sensor.position.x + sensor.distance() - (y - sensor.position.y).abs() + 1;
                    continue 'inner;
                }
            }

            return Ok(x * 4000000 + y);
        }
    }

    todo!()
}

fn parse(input: String) -> Vec<Sensor> {
    let regex =
        Regex::new(r"Sensor at x=(?P<sx>-?\d+), y=(?P<sy>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)")
            .unwrap();

    let mut sensors = Vec::new();

    for line in input.split_terminator('\n') {
        let captures = regex.captures(line).unwrap();

        let sx = captures["sx"].parse().unwrap();
        let sy = captures["sy"].parse().unwrap();
        let bx = captures["bx"].parse().unwrap();
        let by = captures["by"].parse().unwrap();

        let sensor = Sensor::new(Position::new(sx, sy), Position::new(bx, by));
        sensors.push(sensor);
    }

    sensors
}

#[derive(Debug, PartialEq, Eq)]
struct Sensor {
    position: Position,
    beacon: Position,
}

impl Sensor {
    fn new(position: Position, beacon: Position) -> Self {
        Self { position, beacon }
    }

    fn distance(&self) -> i64 {
        self.position.distance(&self.beacon)
    }

    // fn find_edges(&self, rhs: &Self) -> Option<(Position, Position)> {}
}

#[derive(Debug, PartialEq, Eq)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn distance(&self, rhs: &Self) -> i64 {
        (self.x - rhs.x).abs() + (self.y - rhs.y).abs()
    }
}
