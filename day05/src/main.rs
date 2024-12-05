use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let (rules, pages) = parse_input("input/day05.txt")?;

    let part1 = part1(&pages, &rules);
    let part2 = part2(&pages, &rules);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

    Ok(())
}

fn parse_input(file_name: &str) -> std::io::Result<(HashSet<(usize, usize)>, Vec<Vec<usize>>)> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);

    let mut rules: HashSet<(usize, usize)> = HashSet::new();
    let mut pages: Vec<Vec<usize>> = Vec::new();

    let mut buffer = String::new();
    while let Ok(_) = reader.read_line(&mut buffer) {
        if buffer.trim().is_empty() {
            break;
        }
        rules.insert(buffer.split('|').filter(|s| !s.trim().is_empty()).map(|s| {
            let s = s.trim();
            s.parse().unwrap()
        }).collect_tuple().unwrap());
        buffer.clear();
    }

    while let Ok(_) = reader.read_line(&mut buffer) {
        if buffer.trim().is_empty() {
            break;
        }
        pages.push(buffer.trim().split(',').map(|s| s.parse().unwrap()).collect());
        buffer.clear();
    }

    Ok((rules, pages))
}

fn part1(pages: &Vec<Vec<usize>>, rules: &HashSet<(usize, usize)>) -> usize {
    pages.iter().filter(|p| is_valid(&p, &rules))
        .map(|p| p[p.len() / 2]).sum()
}

fn part2(pages: &Vec<Vec<usize>>, rules: &HashSet<(usize, usize)>) -> usize {
    let part2_pages: Vec<Vec<usize>> = pages.clone().into_iter().filter(|p| !is_valid(p, &rules)).collect();
    let mut fixed_pages = Vec::new();
    for p in part2_pages {
        let mut fixed = p.clone();
        while !is_valid(&fixed, &rules) {
            fix_pages(&mut fixed, &rules);
        }
        fixed_pages.push(fixed);
    }

    fixed_pages.iter().map(|p| p[p.len() / 2]).sum()
}

fn fix_pages(pages: &mut Vec<usize>, rules: &HashSet<(usize, usize)>) {
    for combination in pages.clone().iter().combinations(2) {
        if rules.contains(&(*combination[1], *combination[0])) {
            // Need to swap these
            let pos0 = pages.iter().position(|p| p == combination[0]).unwrap();
            let pos1 = pages.iter().position(|p| p == combination[1]).unwrap();
            pages.swap(pos0, pos1);
        }
    }
}

fn is_valid(pages: &[usize], rules: &HashSet<(usize, usize)>) -> bool {
    let mut combinations = HashSet::new();
    for combination in pages.iter().combinations(2) {
        combinations.insert(combination);
    }

    combinations.iter().all(|comb| !rules.contains(&(*comb[1], *comb[0])))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> std::io::Result<()> {
        let (rules, pages) = parse_input("../test_input/day05test.txt")?;
        let part1 = part1(&pages, &rules);
        assert_eq!(part1, 143);

        Ok(())
    }

    #[test]
    fn test_part2() -> std::io::Result<()> {
        let (rules, pages) = parse_input("../test_input/day05test.txt")?;
        let part2 = part2(&pages, &rules);
        assert_eq!(part2, 123);

        Ok(())
    }
}
