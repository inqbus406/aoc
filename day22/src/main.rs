use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input/day22.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut sum = 0;

    for line in lines {
        let Ok(monkey) = line else {
            continue;
        };
        if monkey.is_empty() {
            continue;
        }

        let monkey = monkey.parse::<u64>().unwrap();
        sum += iterations(monkey, 2000);
    }

    println!("Part1: {sum}");

    Ok(())
}

fn iterations(number: u64, n: usize) -> u64 {
    let mut secret_number = number;

    for _ in 0..n {
        secret_number = mix(secret_number, secret_number * 64);
        secret_number = prune(secret_number);

        secret_number = mix(secret_number, secret_number / 32);
        secret_number = prune(secret_number);

        secret_number = mix(secret_number, secret_number * 2048);
        secret_number = prune(secret_number);
    }

    secret_number
}

fn mix(secret_num: u64, given_num: u64) -> u64 {
    secret_num ^ given_num
}

fn prune(number: u64) -> u64 {
    number % 16777216
}
