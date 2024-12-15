use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input/day15.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    while let Ok(n) = reader.read_line(&mut buffer) {
        if n == 2 { // Why does 2 work here? :hmmm:
            // empty line, break
            break;
        }
    }

    let mut map = Map::from_str(&buffer);
    // println!("{}", buffer);
    // dbg!(&map);
    buffer.clear();

    // Get the instructions
    while let Ok(n) = reader.read_line(&mut buffer) {
        if n == 0 {
            // Done with the file
            break;
        }
    }

    // println!("{}", buffer);

    for dir in buffer.chars() {
        if dir.is_whitespace() {
            continue;
        }
        // map.display();
        // println!();
        map.move_robot(dir);
    }

    println!("Part1: {}", map.part1());

    Ok(())
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Map {
    robot: Position,
    boxes: HashSet<Position>,
    walls: HashSet<Position>,
    x_size: usize,
    y_size: usize,
}

impl Map {
    fn from_str(input: &str) -> Self {
        let mut robot = Position { x: 0, y: 0 };
        let mut x_size = 0;
        let mut y_size = 0;
        let mut boxes: HashSet<Position> = HashSet::new();
        let mut walls: HashSet<Position> = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            // println!("Line {}: {}", y, line);
            for (x, c) in line.chars().enumerate() {
                print!("{c}");
                let p = Position { x: x as i32, y: y as i32 };
                match c {
                    '@' => robot = p,
                    'O' => _ = boxes.insert(p),
                    '#' => _ = walls.insert(p),
                    _ => {},
                }

                x_size = max(x + 1, x_size);
            }
            println!();
            y_size = max(y, y_size);
        }

        Self {
            robot,
            boxes,
            walls,
            x_size,
            y_size,
        }
    }

    fn is_valid(&self, pos: &Position) -> bool {
        if pos.x < 0 || pos.y < 0 {
            return false;
        }
        if pos.x >= self.x_size as i32 || pos.y >= self.y_size as i32 {
            return false;
        }
        true
    }

    fn is_wall(&self, pos: &Position) -> bool {
        if !self.is_valid(&pos) {
            panic!();
        }
        self.walls.contains(pos)
            || pos.x == 0
            || pos.y == 0
            || pos.x == self.x_size as i32 - 1
            || pos.y == self.y_size as i32 - 1
    }

    fn move_robot(&mut self, dir: char) {
        let next_pos = Self::new_pos(&self.robot, dir);
        if !self.is_valid(&next_pos) {
            panic!();
        }

        if self.is_wall(&next_pos) {
            return;
        }

        // Check if there's a box there and try to move it if so
        if self.boxes.contains(&next_pos) && !self.move_box(&next_pos, dir) {
            return;
        }

        self.robot = next_pos;
    }

    fn move_box(&mut self, b: &Position, dir: char) -> bool {
        let next_pos = Self::new_pos(b, dir);
        if !self.is_valid(&next_pos) {
            panic!();  // Should never happen with there being walls
        }

        // Check if it's a wall
        if self.is_wall(&next_pos) {
            return false;
        }

        // Check if there's a box there and try to move it if so
        if self.boxes.contains(&next_pos) && !self.move_box(&next_pos, dir) {
            return false;
        }

        self.boxes.insert(next_pos);
        self.boxes.retain(|p| p != b);

        true
    }

    fn new_pos(p: &Position, dir: char) -> Position {
        match dir {
            '^' => Position { x: p.x, y: p.y - 1},
            '>' => Position { x: p.x + 1, y: p.y},
            'v' => Position { x: p.x, y: p.y + 1},
            '<' => Position { x: p.x - 1, y: p.y},
            _   => {
                println!("{dir} is not a valid direction!");
                panic!();
            },
        }
    }

    fn part1(&self) -> i32 {
        self.boxes.iter().map(|b| b.x + b.y * 100).sum()
    }

    fn display(&self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                let p = Position { x: x as i32, y: y as i32 };
                if self.robot == p {
                    print!("@");
                    continue;
                }
                if self.is_wall(&p) {
                    print!("#");
                    continue;
                }
                if self.boxes.contains(&p) {
                    print!("O");
                    continue;
                }
                print!(".");
            }
            println!();
        }
    }
}
