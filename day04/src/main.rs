use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

fn main() -> std::io::Result<()> {
    let f = File::open("input/day04.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines();

    let mut xword = Vec::new();
    let mut result = 0;

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let v = line.chars().collect::<Vec<char>>();

        xword.push(v);
    }

    for (i, line) in xword.iter().enumerate() {
        result += count_xmas(&String::from_iter(line.clone().into_iter()));
        result += count_xmas(&diag_string(0, i, true, &xword));
        result += count_xmas(&diag_string(0, i, false, &xword));
    }
    for j in 1..xword[0].len() {
        result += count_xmas(&diag_string(j, 0, true, &xword));
        result += count_xmas(&diag_string(j, xword.len() - 1, false, &xword));
    }
    for i in 0..xword[0].len() {
        result += count_xmas(&vertical_string(i, &xword));
    }

    println!("Count: {}", result);
    // dbg!(diag_string(0, 8, false, &xword));

    Ok(())
}

fn vertical_string(x: usize, xword: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    for word in xword {
        result.push(word[x]);
    }
    result
}

fn diag_string(x: usize, y: usize, descend: bool, xword: &Vec<Vec<char>>) -> String {
    let mut result = String::new();
    if descend {
        for (i, j) in (x..xword[0].len()).zip(y..xword.len()) {
            result.push(xword[j][i]);
            // println!("{}, {}", i, j);
        }
    } else {
        for (i, j) in (x..xword[0].len()).zip((0..=y).rev()) {
            result.push(xword[j][i]);
            // println!("{}, {}", i, j);
        }
    }
    result
}

fn count_xmas(word: &str) -> usize {
    // println!("checking: {}", word);
    // let re = Regex::new(r"XMAS|SAMX").unwrap();
    // let result = re.captures_iter(word).count();
    let mut result = word.match_indices("XMAS").count();
    result += word.match_indices("SAMX").count();
    // println!("Contains: {}", result);

    result
}
