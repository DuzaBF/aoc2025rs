use crate::read_lines::read_lines;

struct SafeDial {
    position: i32,
    last_val: i32,
    counter: i32,
}

impl SafeDial {
    fn move_dial(&mut self, turn: i32) {
        let new = ((self.position + turn) + self.last_val) % self.last_val;
        self.position = new;
        if self.position == 0 {
            self.counter = self.counter + 1
        }
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
