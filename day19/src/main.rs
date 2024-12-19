use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input/day19.txt")?;
    let reader = BufReader::new(f);
    let mut lines = reader.lines();

    let towels = lines.next().unwrap().unwrap()
        .split(", ")
        .map(|s| String::from(s))
        .collect::<Vec<_>>();

    let mut patterns = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }

        patterns.push(line);
    }

    let part1 = patterns.iter()
        .filter(|p| is_possible(p, &towels))
        .count();
    println!("Part1: {}", part1);

    let mut cached_searcher = Searcher::new();

    let part2 = patterns.iter()
        .map(|p| cached_searcher.possibilities(p, &towels))
        .sum::<usize>();
    println!("Part2: {}", part2);


    Ok(())
}

fn is_possible(pattern: &str, towels: &Vec<String>) -> bool {
    if towels.iter().any(|s| s == pattern) {
        return true;
    }
    for towel in towels {
        match pattern.find(towel) {
            Some(0) => {
                let substring = &pattern[towel.chars().count()..];
                if is_possible(substring, &towels) {
                    return true;
                }
            },
            _ => continue,
        }
    }

    false
}

fn possibilities(pattern: &str, towels: &Vec<String>) -> usize {
    let matching_towels = towels.iter()
        .filter(|s| s.len() == pattern.len())
        .filter(|s| s == &pattern)
        .collect::<HashSet<_>>();
    let mut count = matching_towels.len();
    let remaining_towels = towels.iter()
        .filter(|s| !matching_towels.contains(s))
        .collect::<Vec<_>>();
    for towel in remaining_towels {
        match pattern.find(towel) {
            Some(0) => {
                let substring = &pattern[towel.chars().count()..];
                count += possibilities(substring, towels);
            },
            _ => continue,
        }
    }

    count
}

struct Searcher<'a> {
    cache: HashMap<&'a str, usize>,
}

impl<'a> Searcher<'a> {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    fn possibilities(&mut self, pattern: &'a str, towels: &Vec<String>) -> usize {
        if self.cache.contains_key(pattern) {
            return self.cache[pattern];
        }
        let matching_towels = towels.iter()
            .filter(|s| s.len() == pattern.len())
            .filter(|s| s == &pattern)
            .collect::<HashSet<_>>();
        let mut count = matching_towels.len();
        for towel in towels {
            if matching_towels.contains(&towel) {
                continue;
            }
            match pattern.find(towel) {
                Some(0) => {
                    let substring = &pattern[towel.chars().count()..];
                    count += self.possibilities(substring, towels);
                },
                _ => continue,
            }
        }

        self.cache.insert(pattern, count);

        count
    }
}
