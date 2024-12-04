use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("input/day04.txt")?;
    let reader = BufReader::new(f);

    let lines = reader.lines();

    let mut xword = Vec::new();

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

    part1(&xword);
    part2(&xword);

    Ok(())
}

fn part1(xword: &Vec<Vec<char>>) {
    let mut result = 0;
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

    println!("Part1: {}", result);
}

fn part2(xword: &Vec<Vec<char>>) {
    let mut result = 0;
    for i in 1..(xword[0].len() - 1) {
        for j in 1..(xword.len() - 1) {
            if xword[j][i] == 'A' {
                if check_for_x(i, j, &xword) {
                    result += 1;
                }
            }
        }
    }
    println!("Part2: {}", result);
}

fn check_for_x(x: usize, y: usize, xword: &Vec<Vec<char>>) -> bool {
    if x == 0 || x == xword[0].len() - 1 {
        return false;
    }
    if y == 0 || y == xword.len() - 1 {
        return false;
    }
    match xword[y - 1][x - 1] {
        'M' => match xword[y + 1][x + 1] {
            'S' => {},
            _ => return false,
        }
        'S' => match xword[y + 1][x + 1] {
            'M' => {},
            _ => return false,
        }
        _ => return false,
    }
    match xword[y + 1][x - 1] {
        'M' => match xword[y - 1][x + 1] {
            'S' => {},
            _ => return false,
        }
        'S' => match xword[y - 1][x + 1] {
            'M' => {},
            _ => return false,
        }
        _ => return false,
    }

    true
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
        }
    } else {
        for (i, j) in (x..xword[0].len()).zip((0..=y).rev()) {
            result.push(xword[j][i]);
        }
    }
    result
}

fn count_xmas(word: &str) -> usize {
    let mut result = word.match_indices("XMAS").count();
    result += word.match_indices("SAMX").count();

    result
}
