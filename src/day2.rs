use crate::read_lines::read_lines;
use std::vec::Vec;

fn calc_value(start: u64, i: u32, j: u32) -> u64 {
    let mut value = start;
    for _ in 1..j / i {
        value = value * 10_u64.pow(i) + start;
    }
    // println!("start {} value {}", start, value);
    value
}

fn check(left: u64, right: u64, value: u64, i: u32, j: u32) -> bool {
    if left <= value && value <= right {
        let batch = value % 10_u64.pow(i);
        println!("check {value} is {batch} times {} in {left}-{right}", j/i);
        return true;
    }
    println!("check {value} not in {left}-{right}");
    false
}

fn find_ids(left: u64, right: u64) -> u64 {
    let mut sum: u64 = 0;
    let l_d_c = (left as f32).log10() as u32 + 1;
    let r_d_c = (right as f32).log10() as u32 + 1;
    let r_len = r_d_c / 2;
    println!("left {} {} right {} {}", left, l_d_c, right, r_d_c);
    for i in 1..=r_len {
        'batch_count: for j in l_d_c..=r_d_c {
            if j % i != 0 {
                continue 'batch_count;
            }
            if j / i <= 1 {
                continue 'batch_count;
            }
            let mut start = 10_u64.pow(i - 1);
            let mut ones = 1;
            for _ in 1..i {
                ones = ones * 10 + 1;
            }
            // println!("length {} total {} start {}", i, j, start);
            let mut value = calc_value(start, i, j);
            while value <= right && start < 10_u64.pow(i) {
                if value >= left && value <= right {
                    println!("found {} batch {} i {} j {}", value, start, i, j);
                    check(left, right, value, i,j);
                    sum += value;
                }
                start += 1;
                value = calc_value(start, i, j);

                if start % ones == 0 && i > 1 {
                    start += 1;
                    value = calc_value(start, i, j);
                }
            }
        }
    }
    sum
}

pub fn solution() {
    let mut sum: u64 = 0;
    if let Ok(lines) = read_lines("./input/day2/input") {
        for line in lines.map_while(Result::ok) {
            let ranges: Vec<&str> = line.split(",").collect();
            let slice = &ranges;
            for range in slice {
                let limits: Vec<&str> = range.split("-").collect();
                let left = limits[0].parse::<u64>().unwrap_or(0);
                let right = limits[1].parse::<u64>().unwrap_or(0);
                sum += find_ids(left, right);
            }
        }
    }
    println!("total sum {}", sum);
}
