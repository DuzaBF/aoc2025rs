use crate::read_lines::read_lines;

fn overlap(a: &(u64, u64), b: &(u64, u64)) -> bool {
    if a.0 < b.0 {
        return a.1 >= b.0;
    } else {
        return b.1 >= a.0;
    }
}

fn merge_ranges(ranges: &mut Vec<(u64, u64)>) -> bool {
    let mut has_overlap = false;
    for i in 0..ranges.len() {
        for j in i + 1..ranges.len() {
            if overlap(&ranges[i], &ranges[j]) {
                // println!(
                //     "overlap r[{:3}] = {:16?} and r[{:3}] = {:16?}",
                //     i, ranges[i], j, ranges[j]
                // );
                has_overlap = true;
                ranges[i] = (ranges[i].0.min(ranges[j].0), ranges[i].1.max(ranges[j].1));
                // println!("new r[{:3}] = {:16?}", i, ranges[i]);
                ranges.remove(j);
                return has_overlap;
            }
        }
    }
    has_overlap
}

pub fn solution() {
    let mut sum = 0;
    let mut ranges: Vec<(u64, u64)> = vec![];
    let mut ids: Vec<u64> = vec![];
    let mut divider = false;
    if let Ok(lines) = read_lines("./input/day5/input") {
        for (_, line) in lines.map_while(Result::ok).into_iter().enumerate() {
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

    while merge_ranges(&mut ranges) {}

    ranges.sort_by(|a, b| a.0.cmp(&b.0));

    for r in &ranges {
        let count = r.1 - r.0 + 1;
        sum += count;
    }

    println!("sum {}", sum);
}
