use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/*
const FILENAME: &str = "input-test.txt";
const PART_ONE_Y: i64 = 10;
const PART_ONE_LBOUND: i64 = -100;
const PART_ONE_UBOUND: i64 = 100;
const PART_TWO_LBOUND: i64 = 0;
const PART_TWO_UBOUND: i64 = 20;
*/

const FILENAME: &str = "input.txt";
const PART_ONE_Y: i64 = 2_000_000;
const PART_ONE_LBOUND: i64 = -2_000_000;
const PART_ONE_UBOUND: i64 = 6_000_000;
const PART_TWO_LBOUND: i64 = 0;
const PART_TWO_UBOUND: i64 = 4_000_000;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn manhattan_distance(&self, other: &Self) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }

    pub fn is_at(&self, other: &Self) -> bool {
        (self.x == other.x) && (self.y == other.y)
    }
}

#[derive(Debug)]
struct Beacon {
    location: Point,
}

impl Beacon {
    pub fn new(location: Point) -> Self {
        Self { location }
    }
}

#[derive(Debug)]
struct Sensor {
    location: Point,
    closest_beacon: Beacon,
    beacon_distance: i64,
}

impl Sensor {
    pub fn new(location: Point, closest_beacon: Beacon) -> Self {
        let beacon_distance = location.manhattan_distance(&closest_beacon.location);
        Self {
            location,
            closest_beacon,
            beacon_distance,
        }
    }

    pub fn within_beacon_distance(&self, other: &Point) -> bool {
        self.location.manhattan_distance(other) <= self.beacon_distance
    }

    pub fn border_points(&self) -> Vec<Point> {
        let d = self.beacon_distance + 1;
        let (ox, oy) = (self.location.x, self.location.y); // origin
        (0..d)
            .flat_map(|step| {
                vec![
                    Point::new(ox + step, oy + step - d), // top right border point
                    Point::new(ox + d - step, oy + step), // bottom right border point
                    Point::new(ox - step, oy + d - step), // bottom left border point
                    Point::new(ox + step - d, oy - step), // top left border point
                ]
            })
            .collect()
    }

    pub fn list_from_file(filename: &str) -> Vec<Sensor> {
        let file = File::open(filename).unwrap();
        let io = BufReader::new(file);
        io.lines()
            .map(|l| {
                // Parsing the file is horrific without regex, but I'm trying to stick
                // to the Rust standard library!
                let line = l.unwrap();
                let ints: Vec<i64> = line
                    .match_indices("=")
                    .map(|(p, _)| {
                        let mut substr = &line[p + 1..];
                        if let Some((bad_ix, _)) = substr
                            .match_indices(|c: char| !c.is_numeric() && c != '-')
                            .next()
                        {
                            substr = &substr[..bad_ix]
                        }
                        substr.parse::<i64>().unwrap()
                    })
                    .collect();
                let point = Point::new(ints[0], ints[1]);
                let beacon = Beacon::new(Point::new(ints[2], ints[3]));
                Sensor::new(point, beacon)
            })
            .collect()
    }

    pub fn extract_beacons(sensors: &Vec<Sensor>) -> Vec<&Beacon> {
        let mut uniq: HashMap<Point, &Beacon> = HashMap::new();
        for s in sensors {
            uniq.insert(s.closest_beacon.location, &s.closest_beacon);
        }
        uniq.into_values().collect()
    }
}

pub fn part1() {
    let sensors = Sensor::list_from_file(FILENAME);
    let beacons = Sensor::extract_beacons(&sensors);

    let y = PART_ONE_Y;
    let mut count = 0;
    for x in PART_ONE_LBOUND..=PART_ONE_UBOUND {
        let position = Point::new(x, y);
        if sensors.iter().any(|s| s.within_beacon_distance(&position)) {
            count += 1
        }
        if beacons.iter().any(|b| b.location.is_at(&position)) {
            count -= 1
        }
    }

    println!("Positions that cannot contain a beacon on y={y}: {count}");
}

pub fn part2() {
    let sensors = Sensor::list_from_file(FILENAME);

    for s in &sensors {
        for p in s.border_points() {
            if p.x < PART_TWO_LBOUND
                || p.y < PART_TWO_LBOUND
                || p.x > PART_TWO_UBOUND
                || p.y > PART_TWO_UBOUND
            {
                continue;
            }
            if sensors.iter().any(|s2| s2.within_beacon_distance(&p)) {
                continue;
            }

            println!("Tuning frequency: {}", (p.x * 4_000_000) + p.y);
            return;
        }
    }
}

pub fn main() {
    part1();
    part2();
}
