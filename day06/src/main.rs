use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Position = (i32, i32);

fn main() -> std::io::Result<()> {
    let f = File::open("input/day06.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut obstacles: HashSet<Position> = HashSet::new();
    let mut guard_position: Position = (0, 0);
    let mut x_size = 0;
    let mut y_size = 0;

    for (y, line) in lines.enumerate() {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            x_size = max(x_size, x);
            match c {
                '#' => _ = obstacles.insert((x as i32, y as i32)),
                '^' => guard_position = (x as i32, y as i32),
                _ => continue,
            }
        }
        y_size = max(y_size, y);
    }
    let mut map = Map::new(x_size + 1, y_size + 1, guard_position, &obstacles);

    while map.move_guard() {};

    println!("Part1: {}", map.visited.len());

    Ok(())
}

#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn turn_right(&mut self) {
        match &self {
            Direction::North => *self = Direction::East,
            Direction::East => *self = Direction::South,
            Direction::South => *self = Direction::West,
            Direction::West => *self = Direction::North,
        }
    }
}

#[derive(Debug)]
struct Map {
    guard_pos: Position,
    guard_dir: Direction,
    x_size: usize,
    y_size: usize,
    obstacles: HashSet<Position>,
    visited: HashSet<Position>,
}

impl Map {
    fn new(x_size: usize, y_size: usize, guard_pos: Position, obstacles: &HashSet<Position>) -> Self {
        let mut visited = HashSet::new();
        visited.insert(guard_pos);

        Self {
            guard_pos,
            guard_dir: Direction::North,
            x_size,
            y_size,
            obstacles: obstacles.clone(),
            visited,
        }
    }

    fn move_guard(&mut self) -> bool {
        // println!("Guard position: {:?}", self.guard_pos);
        let pos_update = match self.guard_dir {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        };
        let next_pos = (self.guard_pos.0 + pos_update.0, self.guard_pos.1 + pos_update.1);
        if next_pos.0 < 0 || next_pos.0 >= self.x_size as i32 || next_pos.1 < 0 || next_pos.1 >= self.y_size as i32 {
            // dbg!(&self);
            // println!("Guard ended at {:?}", self.guard_pos);
            return false;
        }
        if self.obstacles.contains(&next_pos) {
            self.guard_dir.turn_right();
            // println!("Turned at: {:?}", self.guard_pos);
            return self.move_guard();
        }
        self.visited.insert(next_pos);
        self.guard_pos = next_pos;
        true
    }
}
