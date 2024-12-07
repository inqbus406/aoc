use std::fs::File;
use std::io::{BufRead, BufReader};

struct Equation(u64, Vec<u64>);

fn main() -> std::io::Result<()> {
    let f = File::open("input/day07.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    let mut equations: Vec<Equation> = Vec::new();

    for line in lines {
        let Ok(line) = line else {
            continue;
        };
        if line.is_empty() {
            continue;
        }
        let line = line.trim();
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let test_val = parts[0][0..parts[0].len() - 1].parse::<u64>().unwrap();
        let numbers = parts[1..].iter().map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        equations.push(Equation(test_val, numbers));
    }

    let part1 = equations.iter().filter(|&e| is_valid(e))
        .map(|e| e.0).sum::<u64>();
    println!("Part1 = {}", part1);

    Ok(())
}

fn is_valid(equation: &Equation) -> bool {
    if equation.0 == equation.1.iter().sum()
        || equation.0 == equation.1.iter().product() {
        return true;
    }
    if equation.1.len() < 2 {
        return false;
    }
    if equation.1.len() == 2 && equation.0 == concat(equation.1[0], equation.1[1]) {
        return true;
    }
    if equation.1.len() == 2 {
        return false;
    }
    let mut vec1 = vec![equation.1[0] + equation.1[1]];
    let mut vec2 = vec![equation.1[0] * equation.1[1]];
    let mut vec3 = vec![concat(equation.1[0],equation.1[1])];
    vec1.extend(&equation.1[2..]);
    vec2.extend(&equation.1[2..]);
    vec3.extend(&equation.1[2..]);

    is_valid(&Equation(equation.0, vec1))
        || is_valid(&Equation(equation.0, vec2))
        || is_valid(&Equation(equation.0, vec3))
}

fn concat(num1: u64, num2: u64) -> u64 {
    num1 * 10_u64.pow(num2.to_string().len() as u32) + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(11, 12), 1112);
        assert_eq!(concat(123, 45), 12345);
    }
}
