use std::cmp::{max, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let mut maze = Maze::from_file("input/day20.txt")?;
    maze.display();

    let shortest_nocheat = maze.shortest_nocheat();
    println!("Shortest nocheat: {}", shortest_nocheat);
    let part1 = maze.cheats_faster_than(shortest_nocheat - 100);
    println!("Part1: {}", part1);

    Ok(())
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i32,
    y: i32,
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
struct Next {
    loc: Position,
    cost: usize,
    cheated: Option<(Position, Position)>,
    path: HashSet<Position>,
}

impl Eq for Next {}

impl PartialEq<Self> for Next {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
            && self.loc == other.loc
    }
}

impl PartialOrd<Self> for Next {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Next {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[derive(Debug)]
struct Maze {
    width: usize,
    height: usize,
    walls: HashSet<Position>,
    visited: HashMap<Position, usize>,  // store minimum cost to get there with visited tiles
    start: Position,
    end: Position,
    cheats: HashSet<(Position, Position)>,
}

impl Maze {
    fn from_file(path: impl AsRef<std::path::Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut width = 0;
        let mut height = 0;
        let mut walls = HashSet::new();
        let mut end = Position { x: 0, y: 0 };
        let mut start = Position { x: 0, y: 0 };

        for (y, line) in lines.enumerate() {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }
            width = line.len();
            for (x, c) in line.chars().enumerate() {
                let p = Position { x: x as i32, y: y as i32 };
                match c {
                    '#' => _ = walls.insert(p),
                    'E' => end = p,
                    'S' => start = p,
                    _   => {}
                }
            }
            height = max(height, y + 1);
        }

        Ok(Self {
            width,
            height,
            walls,
            visited: HashMap::new(),
            start,
            end,
            cheats: HashSet::new(),
        })
    }

    fn cheats_faster_than(&mut self, shortest_nocheat: usize) -> usize {
        self.visited.insert(self.start.clone(), 0);
        let mut current = Next {
            loc: self.visited.keys().nth(0).unwrap().clone(),
            cost: 0,
            cheated: None,
            path: HashSet::new(),
        };

        let mut fringe = VecDeque::new();
        fringe.push_back(current);

        while !fringe.is_empty() {
            current = fringe.pop_front().unwrap();
            self.visited.insert(current.loc, current.cost);
            // self.display();
            // println!("At {:?}, steps: {}, visited: {:?}", current.loc, current.cost, &self.visited);

            // if current.cost > 10 {
            //     loop {}
            // }

            if current.loc == self.end {
                if let Some((cheat_start, cheat_end)) = current.cheated {
                    // println!("Found cheating solution: {:?}->{:?}, cost: {}", cheat_start, cheat_end, current.cost);
                    // Add to set of cheats
                    if current.cost <= shortest_nocheat {
                        println!("Found cheat solution!");
                        self.cheats.insert((cheat_start, cheat_end));
                    }
                }
            }

            for neighbor in self.next_options(&current.loc) {
                if current.path.contains(&neighbor) {
                    continue;
                }

                let mut path = current.path.clone();
                path.insert(neighbor);

                fringe.push_back(Next {
                    loc: neighbor,
                    cost: current.cost + 1,
                    cheated: current.cheated,
                    path,
                });
            }

            if current.cheated.is_some() {
                // Already cheated, can't do it again
                continue;
            }

            // If we can cheat, try it...
            for (cheat_start, cheat_end) in self.next_options_cheat(&current.loc) {
                if self.visited.contains_key(&cheat_end) {
                    if *self.visited.get(&cheat_end).unwrap() > (current.cost + 2) {
                        self.visited.entry(cheat_end).and_modify(|v| *v = current.cost + 2);  // cheating is for 2 steps
                    }
                }

                let cheat = (cheat_start, cheat_end);
                let mut path = current.path.clone();
                path.insert(cheat_end);

                fringe.push_back(Next {
                    loc: cheat_end,
                    cost: current.cost + 2,
                    cheated: Some(cheat),
                    path,
                })
            }
        }

        // dbg!(&self.cheats);

        self.cheats.len()
    }

    fn shortest_nocheat(&mut self) -> usize {
        self.visited.insert(self.start.clone(), 0);
        let mut current = Next {
            loc: self.visited.keys().nth(0).unwrap().clone(),
            cost: 0,
            cheated: None,
            path: HashSet::new(),
        };

        let mut fringe = VecDeque::new();
        fringe.push_back(current);

        let mut shortest_nocheat = usize::MAX;

        while !fringe.is_empty() {
            current = fringe.pop_front().unwrap();
            self.visited.insert(current.loc, current.cost);

            if current.loc == self.end && current.cost <= shortest_nocheat {
                if let Some((cheat_start, cheat_end)) = current.cheated {
                    // Add to set of cheats
                    self.cheats.insert((cheat_start, cheat_end));
                } else if current.cost < shortest_nocheat {
                    shortest_nocheat = current.cost;
                }
            }

            for neighbor in self.next_options(&current.loc) {
                if self.visited.contains_key(&neighbor) {
                    continue;
                }

                fringe.push_back(Next {
                    loc: neighbor,
                    cost: current.cost + 1,
                    cheated: current.cheated,
                    path: HashSet::new(),
                });
            }

        }


        // Reset for another run
        self.visited.clear();
        self.cheats.clear();

        shortest_nocheat

    }

    fn next_options(&self, pos: &Position) -> Vec<Position> {
        [Position { x: pos.x + 1, y: pos.y },
            Position { x: pos.x - 1, y: pos.y },
            Position { x: pos.x, y: pos.y - 1 },
            Position { x: pos.x, y: pos.y + 1 }]
            .into_iter().filter(|p| !self.is_wall(p))
            // .filter(|p| !self.visited.contains(p))  // Need to revisit to find all paths
            .collect::<Vec<Position>>()
    }

    // Returns next places available by cheating and the steps it took (either one or two steps)
    // First two return values are cheat_start and cheat_end
    fn next_options_cheat(&self, pos: &Position) -> Vec<(Position, Position)> {
        let mut wall_neighbors = Vec::new();
        if self.is_wall(&Position { x: pos.x + 1, y: pos.y }) {
            wall_neighbors.push(Position { x: pos.x + 2, y: pos.y });
        }
        if self.is_wall(&Position { x: pos.x - 1, y: pos.y }) {
            wall_neighbors.push(Position { x: pos.x - 2, y: pos.y });
        }
        if self.is_wall(&Position { x: pos.x, y: pos.y + 1 }) {
            wall_neighbors.push(Position { x: pos.x, y: pos.y + 2 });
        }
        if self.is_wall(&Position { x: pos.x, y: pos.y - 1}) {
            wall_neighbors.push(Position { x: pos.x, y: pos.y - 2 });
        }

        wall_neighbors.into_iter()
            .filter(|p| self.is_valid(p))
            .filter(|p| !self.is_wall(p))
            .filter(|&end| !self.cheats.contains(&(*pos, end)))
            .map(|end| (pos.clone(), end))
            .collect::<Vec<_>>()
    }

    fn is_valid(&self, p: &Position) -> bool {
        p.x >= 0
            && p.y >= 0
            && p.x < self.width as i32
            && p.y < self.height as i32
    }

    fn is_wall(&self, p: &Position) -> bool {
        self.walls.contains(&p)
    }

    #[allow(dead_code)]
    fn display(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let p = Position { x: x as i32, y: y as i32 };
                if self.is_wall(&p) {
                    print!("#");
                    continue;
                }
                if p == self.start {
                    print!("S");
                    continue;
                }
                if p == self.end {
                    print!("E");
                    continue;
                }
                if self.visited.contains_key(&p) {
                    print!("x");
                    continue;
                }
                print!(".");
            }
            println!();
        }
    }
}
