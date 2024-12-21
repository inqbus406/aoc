use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let f = File::open("test_input/day21test.txt")?;
    let reader = BufReader::new(f);
    let lines = reader.lines();

    for line in lines {
        let Ok(code) = line else {
            continue;
        };
        if code.is_empty() {
            continue;
        }
        let mut keypad_robot = NumericKeypadRobot::new();
        let mut result = String::new();
        for c in code.chars() {
            result.push_str(&keypad_robot.enter_digit(&NumericKey::from_char(c)));
            // result.push_str(&format!("   making {}: {}      ", c, keypad_robot.enter_digit(&NumericKey::from_char(c))));
        }
        println!("{}: {}, len: {}", code, result, result.len());
    }


    Ok(())
}

#[derive(Debug, Clone)]
enum NumericKey {
    A,
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl NumericKey {
    fn from_char(c: char) -> Self {
        match c {
            'A' => NumericKey::A,
            '0' => NumericKey::Zero,
            '1' => NumericKey::One,
            '2' => NumericKey::Two,
            '3' => NumericKey::Three,
            '4' => NumericKey::Four,
            '5' => NumericKey::Five,
            '6' => NumericKey::Six,
            '7' => NumericKey::Seven,
            '8' => NumericKey::Eight,
            '9' => NumericKey::Nine,
            _   => unreachable!(),
        }
    }

    fn position(&self) -> (usize, usize) {
        match self {
            NumericKey::A => (2, 3),
            NumericKey::Zero => (1, 3),
            NumericKey::One => (0, 2),
            NumericKey::Two => (1, 2),
            NumericKey::Three => (2, 2),
            NumericKey::Four => (0, 1),
            NumericKey::Five => (1, 1),
            NumericKey::Six => (2, 1),
            NumericKey::Seven => (0, 0),
            NumericKey::Eight => (1, 0),
            NumericKey::Nine => (2, 0),
        }
    }
}

#[derive(Debug, Clone)]
enum TClusterKey {
    A,
    Right,
    Down,
    Left,
    Up,
}

impl TClusterKey {
    fn position(&self) -> (usize, usize) {
        match self {
            TClusterKey::A => (2, 0),
            TClusterKey::Right => (2, 1),
            TClusterKey::Down => (1, 1),
            TClusterKey::Left => (0, 1),
            TClusterKey::Up => (1, 0),
        }
    }

    fn to_char(&self) -> char {
        match self {
            TClusterKey::A => 'A',
            TClusterKey::Right => '>',
            TClusterKey::Down => 'v',
            TClusterKey::Left => '<',
            TClusterKey::Up => '^',
        }
    }
}

struct NumericKeypadRobot {
    position: NumericKey,
    robot: TClusterKeypadRobot1,
}

struct TClusterKeypadRobot1 {
    position: TClusterKey,
    robot: TClusterKeypadRobot2,
}

struct TClusterKeypadRobot2 {
    position: TClusterKey,
}

impl NumericKeypadRobot {
    fn new() -> Self {
        Self {
            position: NumericKey::A,
            robot: TClusterKeypadRobot1::new(),
        }
    }

    fn enter_digit(&mut self, digit: &NumericKey) -> String {
        // println!("Keypad robot wants {:?}", digit);
        let mut result = String::new();
        for key in self.moves_to_digit(digit) {
            // println!("Keypad robot asks for {:?}", key);
            result.push_str(&self.robot.enter_direction(&key));
        }
        self.position = digit.clone();

        result
    }

    fn moves_to_digit(&self, digit: &NumericKey) -> Vec<TClusterKey> {
        // Always move right before down and up before left to avoid the gap
        let start = self.position.position();
        let end = digit.position();

        let mut result = Vec::new();
        // let mut temp = start.clone();
        //
        // while temp != end {
        //     // println!("Hanging here, temp: {:?}, end: {:?}", &temp, &end);
        //
        //     if temp.1 > end.1 {
        //         result.push(TClusterKey::Up);
        //         temp.1 -= 1;
        //     }
        //
        //     if temp.0 > end.0 {
        //         result.push(TClusterKey::Left);
        //         temp.0 -= 1;
        //     } else if temp.0 < end.0 {
        //         result.push(TClusterKey::Right);
        //         temp.0 += 1;
        //     }
        //
        //     if temp.1 < end.1 {
        //         result.push(TClusterKey::Down);
        //         temp.1 += 1;
        //     }
        // }

        let moving_up = start.1 > end.1;
        let moving_left = start.0 > end.0;

        if moving_up {
            for _ in 0..start.1.abs_diff(end.1) {
                result.push(TClusterKey::Up);
            }
        }

        if moving_left {
            for _ in 0..start.0.abs_diff(end.0) {
                result.push(TClusterKey::Left);
            }
        } else {
            for _ in 0..start.0.abs_diff(end.0) {
                result.push(TClusterKey::Right);
            }
        }

        if !moving_up {
            for _ in 0..start.1.abs_diff(end.1) {
                result.push(TClusterKey::Down);
            }
        }

        // Always add an A at the end
        result.push(TClusterKey::A);

        result
    }
}

impl TClusterKeypadRobot1 {
    fn new() -> Self {
        Self {
            position: TClusterKey::A,
            robot: TClusterKeypadRobot2::new(),
        }
    }

    fn enter_direction(&mut self, direction: &TClusterKey) -> String {
        // println!("TClusterKeypadRobot1 wants {:?}", direction);
        let mut result = String::new();
        for key in self.moves_to_key(direction) {
            // println!("TClusterKeypadRobot1 asks for {:?}", key);
            result.push_str(&self.robot.enter_direction(&key));
        }

        self.position = direction.clone();

        result
    }

    fn moves_to_key(&self, key: &TClusterKey) -> Vec<TClusterKey> {
        // Always move right before up and down before left
        let mut result = Vec::new();
        let start = self.position.position();
        let end = key.position();

        let moving_up = start.1 > end.1;
        let moving_left = start.0 > end.0;

        if !moving_up {
            for _ in 0..start.1.abs_diff(end.1) {
                result.push(TClusterKey::Down);
            }
        }

        if moving_left {
            for _ in 0..start.0.abs_diff(end.0) {
                result.push(TClusterKey::Left);
            }
        } else {
            for _ in 0..start.0.abs_diff(end.0) {
                result.push(TClusterKey::Right);
            }
        }

        if moving_up {
            for _ in 0..start.1.abs_diff(end.1) {
                result.push(TClusterKey::Up);
            }
        }

        result.push(TClusterKey::A);
        result
    }
}

impl TClusterKeypadRobot2 {
    fn new() -> Self {
        Self {
            position: TClusterKey::A,
        }
    }

    fn enter_direction(&mut self, direction: &TClusterKey) -> String {
        // println!("TClusterKeypadRobot2 wants {:?}", direction);
        let mut result = String::new();
        for key in self.moves_to_key(direction) {
            // println!("TClusterKeypadRobot2 asks for {:?}", key);
            result.push(key.to_char());
        }
        self.position = direction.clone();

        result
    }

    // This function will probably be wrapped in a Trait or something because it's shared
    fn moves_to_key(&self, key: &TClusterKey) -> Vec<TClusterKey> {
        let mut result = Vec::new();
        let start = self.position.position();
        let end = key.position();

        let moving_up = start.1 > end.1;
        let moving_left = start.0 > end.0;

        if !moving_up {
            for _ in 0..start.1.abs_diff(end.1) {
                result.push(TClusterKey::Down);
            }
        }

        if moving_left {
            for _ in 0..start.0.abs_diff(end.0) {
                result.push(TClusterKey::Left);
            }
        } else {
            for _ in 0..start.0.abs_diff(end.0) {
                result.push(TClusterKey::Right);
            }
        }

        if moving_up {
            for _ in 0..start.1.abs_diff(end.1) {
                result.push(TClusterKey::Up);
            }
        }

        result.push(TClusterKey::A);
        result
    }
}

