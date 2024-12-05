use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;

fn main() -> std::io::Result<()> {
    let f = File::open("input/day05.txt")?;
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

    // dbg!(&rules);
    let part1: usize = pages.iter().filter(|p| is_valid(&p, &rules))
        .map(|p| p[p.len() / 2]).sum();
    println!("Part 1: {}", part1);
    // for p in pages {
    //     if is_valid(&p, &rules) {
    //         dbg!(&p);
    //     } else {
    //         println!("Not valid");
    //     }
    // }

    let part2_pages: Vec<Vec<usize>> = pages.into_iter().filter(|p| !is_valid(&p, &rules)).collect();
    let mut fixed_pages = Vec::new();
    for p in part2_pages {
        let mut fixed = p.clone();
        while !is_valid(&fixed, &rules) {
            fix_pages(&mut fixed, &rules);
        }
        fixed_pages.push(fixed);
    }
    // dbg!(&fixed_pages);
    let part2: usize = fixed_pages.iter().map(|p| p[p.len() / 2]).sum();
    println!("Part 2: {}", part2);

    Ok(())
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

    // dbg!(&pages);
    let mut combinations = HashSet::new();
    for combination in pages.iter().combinations(2) {
        combinations.insert(combination);
    }
    // dbg!(&rules);
    // for perm in &combinations {
    //     if rules.contains(&(*perm[1], *perm[0])) {
    //         println!("{:?} against the rules", perm);
    //     }
    // }

    // dbg!(&combinations);

    combinations.iter().all(|comb| !rules.contains(&(*comb[1], *comb[0])))
}
