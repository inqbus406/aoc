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
