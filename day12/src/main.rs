use std::error::Error;
use std::fmt::Display;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file = &fs::read("input.txt")?;
    let input_as_str = String::from_utf8_lossy(&input_file);

    let commands: Vec<Command> = input_as_str
        .lines()
        .map(|line| {
            let (cmd, amt) = line.split_at(1);
            let amount: i32 = amt.parse().unwrap();
            match cmd {
                "N" => Command::North(amount),
                "S" => Command::South(amount),
                "E" => Command::East(amount),
                "W" => Command::West(amount),
                "L" => Command::Left(amount),
                "R" => Command::Right(amount),
                "F" => Command::Forward(amount),
                _ => panic!(),
            }
        })
        .collect();

    part1(&commands);
    part2(&commands);
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Direction::North => "N",
                Direction::South => "S",
                Direction::East => "E",
                Direction::West => "W",
            }
        )
    }
}

impl Direction {
    fn rotated(&self, degrees: i32) -> Direction {
        let to_rotate = degrees % 360;
        let rotations = to_rotate / 90;

        let directions = vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ];
        let cur_pos = directions.iter().position(|d| d == self).unwrap();

        let new_pos = (cur_pos + (rotations as usize)) % 4;

        return directions[new_pos];
    }
}

#[derive(Debug, Copy, Clone)]
enum Command {
    North(i32),
    South(i32),
    East(i32),
    West(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}

struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn apply(&mut self, cmd: &Command) {
        match cmd {
            Command::North(n) => self.y += n,
            Command::South(n) => self.y -= n,
            Command::East(n) => self.x += n,
            Command::West(n) => self.x -= n,
            Command::Left(deg) => self.apply(&Command::Right(360 - *deg)),
            Command::Right(deg) => {
                let real_deg = deg % 360;
                let rotations = real_deg / 90;
                for _ in 0..rotations {
                    let prev_x = self.x;
                    self.x = self.y;
                    self.y = -prev_x;
                }
            }
            _ => panic!(),
        }
    }
}

struct Location {
    point: Point,
    facing: Direction,
}

impl Location {
    fn apply(&mut self, cmd: &Command) {
        match cmd {
            Command::North(_) | Command::South(_) | Command::East(_) | Command::West(_) => {
                self.point.apply(&cmd)
            }
            Command::Left(deg) => self.facing = self.facing.rotated(360 - *deg),
            Command::Right(deg) => self.facing = self.facing.rotated(*deg),
            Command::Forward(distance) => self.apply(&match self.facing {
                Direction::North => Command::North(*distance),
                Direction::South => Command::South(*distance),
                Direction::East => Command::East(*distance),
                Direction::West => Command::West(*distance),
            }),
        }
    }
}

struct Location2 {
    point: Point,
    waypoint: Point,
}

impl Location2 {
    fn apply(&mut self, cmd: &Command) {
        match cmd {
            Command::Forward(distance) => {
                let to_move_x = self.waypoint.x * distance;
                let to_move_y = self.waypoint.y * distance;
                self.point.x += to_move_x;
                self.point.y += to_move_y;
            }
            _ => self.waypoint.apply(&cmd),
        }
    }
}

fn part1(commands: &Vec<Command>) {
    let mut position = Location {
        point: Point { x: 0, y: 0 },
        facing: Direction::East,
    };

    for cmd in commands {
        position.apply(&cmd);
    }
    let distance = position.point.x.abs() + position.point.y.abs();

    println!("Part1: final distance: {}", distance);
}

fn part2(commands: &Vec<Command>) {
    let mut position = Location2 {
        point: Point { x: 0, y: 0 },
        waypoint: Point { x: 10, y: 1 },
    };

    for cmd in commands {
        position.apply(&cmd);
    }
    let distance = position.point.x.abs() + position.point.y.abs();

    println!("Part2: final distance: {}", distance);
}
