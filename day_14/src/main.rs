use std::env;
use std::fs;
use std::cmp::max;

struct Reindeer {
    speed: u32,
    burst_time: u32,
    rest_time: u32,

    distance: u32,
    points: u32,
    is_running: bool,
    remaining_time: u32,
}

impl Reindeer {
    fn new(speed: u32, burst_time: u32, rest_time: u32) -> Self {
        Self {
            speed: speed,
            burst_time: burst_time,
            rest_time: rest_time,

            distance: 0,
            points: 0,
            is_running: true,
            remaining_time: burst_time,
        }
    }

    fn step(&mut self) {
        self.remaining_time -= 1;

        if self.is_running {
            self.distance += self.speed;
            if self.remaining_time == 0 {
                self.is_running = false;
                self.remaining_time = self.rest_time;
            }
        } else {
            if self.remaining_time == 0 {
                self.is_running = true;
                self.remaining_time = self.burst_time;
            }
        }
    }
}

fn main() {
    let time: Vec<String> = env::args().collect();
    let time = time[1].parse().unwrap();

    let input = fs::read_to_string("../input.txt").expect("Failed to read file");
    let mut reindeers = parse(input.trim());

    let (distance, points) = calc_winners(&mut reindeers, time);

    println!("Part 1: the best reindeer crosses {} kilometers", distance);
    println!("Part 2: the best reindeer got {} points", points);
}

fn parse(input: &str) -> Vec<Reindeer> {
    let mut reindeers = Vec::new();

    for line in input.split('\n') {
        let words: Vec<&str> = line.split(' ').collect();
        let reindeer = Reindeer::new(
            words[3].parse().unwrap(),
            words[6].parse().unwrap(),
            words[13].parse().unwrap(),
        );
        reindeers.push(reindeer);
    }

    reindeers
}

fn calc_winners(reindeers: &mut Vec<Reindeer>, time: u32) -> (u32, u32) {
    let mut max_distance = 0;
    let mut max_points = 0;

    for _ in 0..time {
        max_distance = 0;

        for r in &mut *reindeers {
            r.step();
            max_distance = max(max_distance, r.distance);
        }

        for r in &mut *reindeers {
            if r.distance == max_distance {
                r.points += 1;
                max_points = max(max_points, r.points);
            }
        }

    }

    (max_distance, max_points)
}
