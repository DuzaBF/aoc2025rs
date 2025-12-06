use crate::read_lines::read_lines;

fn get_jolt(line: &str) -> u32 {
    let mut max: u32 = 0;
    let mut max_pos: usize = 0;
    for (i, ch) in line.chars().into_iter().enumerate() {
        if i == line.len() - 1 {
            continue;
        }
        let num =  ch.to_digit(10).unwrap();
        if num > max {
            max = num;
            max_pos = i;
        }
    }
    let mut sec: u32 = 0;
    let mut sec_pos: usize = 0;
    for (i, ch) in line.chars().into_iter().enumerate() {
        if i <= max_pos {
            continue;
        }
        let num =  ch.to_digit(10).unwrap();
        if num > sec {
            sec = num;
            sec_pos = i;
        }
    }
    10 * max + sec
}

pub fn solution() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./input/day3/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            sum += get_jolt(&line);
            println!("bank {}: {} jolt {}", i, line, get_jolt(&line));
        }
    }
    println!("sum {}", sum);
}
