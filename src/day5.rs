use crate::read_lines::read_lines;

pub fn solution() {
    let mut sum = 0;
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ids: Vec<(u64)> = vec![];
    let mut spoiled: Vec<(u64)> = vec![];
    let mut divider = false;
    if let Ok(lines) = read_lines("./input/day5/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            if line.len() == 0 {
                divider = true;
                continue;
            }
            if divider {
                ids.push(line.parse::<u64>().unwrap_or(0));
            } else {
                let limits: Vec<&str> = line.split("-").collect();
                let left = limits[0].parse::<u64>().unwrap_or(0);
                let right = limits[1].parse::<u64>().unwrap_or(0);
                ranges.push((left, right));
            }
        }
    }

    for id in &ids {
        let mut spoil = true;
        for range in &ranges {
            if *id >= range.0 && *id <= range.1 {
                spoil = false;
            }
        }
        if !spoil {
            spoiled.push(*id);
        }
    }

    sum += spoiled.len();

    println!("sum {}", sum);
}
