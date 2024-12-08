use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Position(i32, i32);

impl Position {
    pub(crate) fn clone(&self) -> Self {
        Position(self.0, self.1)
    }
}

type Map = HashMap<char, Vec<Position>>;

fn main() -> std::io::Result<()> {
    let f = File::open("input/day08.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut map = HashMap::new();
    let mut y_max = 0;
    let mut x_max = 0;

    for (y, line) in lines.enumerate() {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }

        for (x, c) in line.chars().enumerate() {
            match c {
                'a'..='z' | 'A'..='Z' |  '0'..='9' => map.entry(c).or_insert(Vec::new())
                    .push(Position(x.try_into().unwrap(), y.try_into().unwrap())),
                _ => {},
            }
            x_max = max(x_max, x);
        }
        y_max = max(y_max, y);
    }

    println!("Part1: {}", part1(&map, x_max, y_max));
    println!("Part1: {}", part2(&map, x_max, y_max));

    Ok(())
}

// Count antinodes for each frequency
fn part1(map: &Map, x_max: usize, y_max: usize) -> usize {
    let mut result = HashSet::new();
    // Check each pair of each frequency and add their antinodes
    for v in map.values() {
        for pair in v.iter().combinations(2) {
            let (antinode0, antinode1) = find_antinodes(pair[0], pair[1]);
            if in_map(&antinode0, x_max as i32, y_max as i32) {
                result.insert(antinode0);
            }
            if in_map(&antinode1, x_max as i32, y_max as i32) {
                result.insert(antinode1);
            }
        }
    }

    result.len()
}

fn part2(map: &Map, x_max: usize, y_max: usize) -> usize {
    let mut result = HashSet::new();
    for v in map.values() {
        for pair in v.iter().combinations(2) {
            for antinode in find_harmonic_antinodes(pair[0], pair[1], x_max as i32, y_max as i32) {
                result.insert(antinode);
            }
        }
    }

    result.len()
}

fn find_antinodes(p0: &Position, p1: &Position) -> (Position, Position) {
    let diff = (p1.0 - p0.0, p1.1 - p0.1);
    (Position(p0.0 - diff.0, p0.1 - diff.1), Position(p1.0 + diff.0, p1.1 + diff.1))
}

fn find_harmonic_antinodes(p0: &Position, p1: &Position, x_max: i32, y_max: i32) -> Vec<Position> {
    let mut result = vec![p0.clone(), p1.clone()];
    let diff = (p1.0 - p0.0, p1.1 - p0.1);

    let mut temp = p0.clone();
    loop {
        temp.0 -= diff.0;
        temp.1 -= diff.1;
        if !in_map(&temp, x_max, y_max) {
            break;
        }
        result.push(temp.clone());
    }
    temp = p1.clone();
    loop {
        temp.0 += diff.0;
        temp.1 += diff.1;
        if !in_map(&temp, x_max, y_max) {
            break;
        }
        result.push(temp.clone());
    }

    result
}

fn in_map(p: &Position, x_max: i32, y_max: i32) -> bool {
    if p.0 > x_max || p.1 > y_max {
        return false;
    }
    if p.0 < 0 || p.1 < 0 {
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_antinodes() {
        assert_eq!(find_antinodes(&Position(4, 3), &Position(5, 5)), (Position(3, 1), Position(6, 7)));
        assert_eq!(find_antinodes(&Position(4, 3), &Position(8, 4)), (Position(0, 2), Position(12, 5)));
        assert_eq!(find_antinodes(&Position(8, 4), &Position(5, 5)), (Position(11, 3), Position(2, 6)));
        assert_eq!(find_antinodes(&Position(6, 5), &Position(8, 8)), (Position(4, 2), Position(10, 11)));
        assert_eq!(find_antinodes(&Position(8, 8), &Position(6, 5)), (Position(10, 11), Position(4, 2)));
    }

    #[test]
    fn test_in_map() {
        assert!(in_map(&Position(0, 0), 5, 5));
        assert!(!in_map(&Position(0, -1), 5, 5));
        assert!(!in_map(&Position(6, 0), 5, 5));
    }
}
