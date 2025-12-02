use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

struct SafeDial {
    position: i32,
    last_val: i32,
    counter: i32,
}

impl SafeDial {
    fn move_dial(&mut self, turn: i32) {
        let full_turns = (turn / self.last_val).abs();
        let rest_turns = turn % self.last_val;
        let new = ((self.position + rest_turns) + self.last_val) % self.last_val;

        let over_zero = match rest_turns > 0 {
            true => (self.position + rest_turns) / self.last_val,
            false => {
                if self.position == 0 {
                    0
                } else {
                    ((self.position + rest_turns) <= 0) as i32
                }
            }
        };
        self.counter = self.counter + full_turns + over_zero;
        self.position = new;
    }
    fn new(start: i32, last_val: i32) -> Self {
        SafeDial {
            position: start,
            last_val: last_val,
            counter: 0,
        }
    }
    fn get_counter(&self) -> i32 {
        self.counter
    }
}

pub fn solution() {
    let mut dial = SafeDial::new(50, 100);
    if let Ok(lines) = read_lines("./input/day1/input") {
        for line in lines.map_while(Result::ok) {
            let is_left = line.chars().next().unwrap_or(' ') == 'L';
            let value = line[1..].parse::<i32>().unwrap_or(0);
            dial.move_dial(value * ([1, -1][is_left as usize]));
        }
    }
    println!("answer: {}", dial.get_counter())
}
