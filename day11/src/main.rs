use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut stones = Stones::from_file("input/day11.txt")?;

    // dbg!(&stones);
    for _ in 0..75 {
        stones.update();
        // dbg!(&stones);
    }


    println!("Part1: {}", stones.stones.len());

    Ok(())
}

#[derive(Debug)]
struct Stones {
    stones: Vec<u32>,
}

impl Stones {
    fn from_file(path: impl AsRef<Path>) -> std::io::Result<Self> {
        let f = File::open(path)?;
        let reader = BufReader::new(f);
        let lines = reader.lines();

        let mut stones = Vec::new();

        for line in lines {
            let Ok(line) = line else {
                continue;
            };
            if line.is_empty() {
                continue;
            }

            line.trim().split_whitespace().for_each(|x| stones.push(x.parse::<u32>().unwrap()));
        }

        Ok(Self { stones })
    }

    fn update(&mut self) {
        self.stones = self.stones.iter().flat_map(|&num| Self::update_one_num(num)).collect();
    }

    fn update_one_num(num: u32) -> Vec<u32> {
        if num == 0 {
            return vec![1];
        }
        let num_string = num.to_string();
        if num_string.len() % 2 == 0 {
            return vec![num_string[0..num_string.len() / 2].parse().unwrap(), num_string[num_string.len() / 2..].parse().unwrap()];
        }
        vec![num * 2024]
    }
}
