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
    let mut part1: usize = pages.iter().filter(|p| is_valid(&p, &rules))
        .map(|p| p[p.len() / 2]).sum();
    println!("Part 1: {}", part1);
    // for p in pages {
    //     if is_valid(&p, &rules) {
    //         dbg!(&p);
    //     } else {
    //         println!("Not valid");
    //     }
    // }

    Ok(())
}

fn is_valid(pages: &[usize], rules: &HashSet<(usize, usize)>) -> bool {

    // dbg!(&pages);
    let mut combinations = HashSet::new();
    for perm in pages.iter().combinations(2) {
        combinations.insert(perm);
    }
    // dbg!(&rules);
    // for perm in &combinations {
    //     if rules.contains(&(*perm[1], *perm[0])) {
    //         println!("{:?} against the rules", perm);
    //     }
    // }

    // dbg!(&combinations);

    combinations.iter().all(|perm| !rules.contains(&(*perm[1], *perm[0])))
}
