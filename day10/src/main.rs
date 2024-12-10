use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Map = Vec<Vec<u32>>;

#[derive(Debug, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub(crate) fn clone(&self) -> Self {
        Self {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("input/day10.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut map = Map::new();
    let mut trailheads = Vec::new();

    for (y, line) in lines.enumerate() {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        for (x, c) in line.chars().enumerate() {
            let num = c.to_digit(10).unwrap();
            if num == 0 {
                trailheads.push(Point{x: x as i32, y: y as i32});
            }
        }
        map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
    }

    let part1 = part1(&trailheads, &map);
    let part2 = part2(&trailheads, &map);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn part1(trailheads: &Vec<Point>, map: &Map) -> usize {
    trailheads.iter().map(|p| explore(p, map).iter().count()).sum()
}

fn part2(trailheads: &Vec<Point>, map: &Map) -> usize {
    trailheads.iter().map(|p| explore_part2(p, map)).sum()
}

// Returns 9s reachable via this starting point
fn explore(start: &Point, map: &Map) -> HashSet<Point> {
    if !in_bounds(start, map) {
        // Out of bounds
        return HashSet::new();
    }
    if lookup(start, map) == 9 {
        let mut result = HashSet::new();
        result.insert(start.clone());
        return result;
    }
    let to_explore = get_neighbors(start, map).into_iter().filter(|p|
        in_bounds(p, map) && passable(start, p, map)).collect::<Vec<_>>();

    let mut reachable_nines = HashSet::new();
    for p in to_explore {
        for nine in explore(&p, map) {
            reachable_nines.insert(nine);
        }
    }

    reachable_nines
}

fn explore_part2(start: &Point, map: &Map) -> usize {
    if !in_bounds(start, map) {
        // Out of bounds
        return 0;
    }
    if lookup(start, map) == 9 {
        return 1;
    }
    let to_explore = get_neighbors(start, map).into_iter().filter(|p|
        in_bounds(p, map) && passable(start, p, map)).collect::<Vec<_>>();

    to_explore.iter().map(|p| explore_part2(&p, map)).sum()
}

fn get_neighbors(start: &Point, map: &Map) -> Vec<Point> {
    vec![Point{x: start.x + 1, y: start.y},
         Point{x: start.x - 1, y: start.y},
         Point{x: start.x, y: start.y - 1},
         Point{x: start.x, y: start.y + 1}]
}

fn passable(point0: &Point, point1: &Point, map: &Map) -> bool {
    lookup(point1, map) == lookup(point0, map) + 1
}

fn in_bounds(point: &Point, map: &Map) -> bool {
    point.x >= 0 && point.x < map[0].len() as i32 && point.y >= 0 && point.y < map.len() as i32
}

fn lookup(point: &Point, map: &Map) -> u32 {
    if !in_bounds(point, map) {
        // Out of bounds
        panic!();
    }
    map[point.y as usize][point.x as usize]
}
