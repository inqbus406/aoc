use std::cmp::max;
use std::collections::HashSet;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let maze = Maze::from_file("input/day16.txt")?;

    // dbg!(&maze);
    // maze.display();
    // return Ok(());
    let part1 = maze.solve();
    println!("Part1: {}", part1);

    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
    None,
}

impl Direction {
    // Get a number value associated with each direction to calculate angular offset
    fn value(&self) -> u8 {
        match self {
            Direction::North => 1,
            Direction::East => 2,
            Direction::South => 3,
            Direction::West => 4,
            Direction::None => panic!(),
        }
    }

    fn turns_to(&self, other: &Direction) -> usize {
        let difference = self.value().abs_diff(other.value());
        if difference == 2 {
            return 2;
        }
        (difference % 2) as usize
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
    dir: Direction,
}

impl PartialEq for Position {
    // Don't compare direction for same position
    fn eq(&self, other: &Position) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {}

impl Hash for Position {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.x.hash(h);
        self.y.hash(h);
    }
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    walls: HashSet<Position>,
    visited: Vec<Position>,
    end: Position,
    score: usize,
}

impl Maze {
    fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();
        let mut visited = Vec::new();
        let mut end = Position { x: 0, y: 0, dir: Direction::None };

        for (y, line) in lines.enumerate() {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                let p = Position { x: x as i32, y: y as i32, dir: Direction::None };
                match c {
                    '#' => _ = walls.insert(p),
                    'E' => end = p,
                    'S' => visited.push(Position{x: p.x, y: p.y, dir: Direction::East}),
                    _   => {}
                }
            }
            height = max(height, y + 1);
        }

        Ok(Self {
            width,
            height,
            walls,
            visited,
            end,
            score: 0,
        })
    }

    fn solve(&self) -> usize {
        let current = self.visited.last().unwrap();
        // println!("Exploring: {:?}", current);
        if *current == self.end {
            return self.score;
        }
        self.next_options().iter()
            .map(|option| option.solve())
            // .for_each(|option| println!("Option: {:?}", option));
            .min()
            .unwrap_or(usize::MAX)
    }

    fn next_options(&self) -> Vec<Self> {
        let mut options = Vec::new();
        let current = self.visited.last().unwrap();
        let neighbors = [Position { x: current.x + 1, y: current.y, dir: Direction::East },
                                    Position { x: current.x - 1, y: current.y, dir: Direction::West },
                                    Position { x: current.x, y: current.y - 1, dir: Direction::North },
                                    Position { x: current.x, y: current.y + 1, dir: Direction::South }]
            .into_iter().filter(|p| !self.is_wall(p))
            .filter(|p| !self.visited.contains(p))
            .collect::<Vec<Position>>();

        for neighbor in neighbors {
            let mut visited = self.visited.clone();
            visited.push(neighbor);

            options.push(Self {
                width: self.width,
                height: self.height,
                walls: self.walls.clone(),
                visited,
                end: self.end,
                score: self.score + 1 + current.dir.turns_to(&neighbor.dir) * 1000,
            })
        }


        options
    }

    fn is_wall(&self, p: &Position) -> bool {
        self.walls.contains(&p)
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Position { x: x as i32, y: y as i32, dir: Direction::None };
                if self.is_wall(&p) {
                    print!("#");
                    continue;
                }
                print!(".");
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_turns_to() {
        assert_eq!(Direction::North.turns_to(&Direction::North), 0);
        assert_eq!(Direction::North.turns_to(&Direction::East), 1);
        assert_eq!(Direction::North.turns_to(&Direction::South), 2);
        assert_eq!(Direction::North.turns_to(&Direction::West), 1);
        assert_eq!(Direction::East.turns_to(&Direction::West), 2);
        assert_eq!(Direction::East.turns_to(&Direction::South), 1);
        assert_eq!(Direction::East.turns_to(&Direction::North), 1);
    }
}
