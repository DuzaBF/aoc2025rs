use crate::read_lines::read_lines;
use std::vec::Vec;

fn find_ids(left: u64, right: u64) -> u64 {
    let mut sum: u64 = 0;
    let digits_count = (left as f32).log10() as u32 + 1;
    let length = digits_count / 2 + digits_count % 2;
    let r_d_c = (right as f32).log10() as u32 + 1;
    let r_len = r_d_c / 2;
    println!("left {} {} right {} {}", left, digits_count, right, r_d_c);
    for i in length..=r_len {
        let mut start = left.max(10_u64.pow(i * 2 - 1)) / 10_u64.pow(i);
        println!("length {} start {}", i, start);
        let mut value = start * 10_u64.pow(i as u32) + start;
        while value <= right && start < 10_u64.pow(i) {
            println!("start {} value {}", start, value);
            if value >= left && value <= right {
                sum += value as u64;
                println!("sum {}", sum);
            }
            start += 1;
            value = start * 10_u64.pow(i as u32) + start;
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
                println!("running sum {}", sum);
            }
        }
    }
    println!("total sum {}", sum);
}
