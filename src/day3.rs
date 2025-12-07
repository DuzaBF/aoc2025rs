use crate::read_lines::read_lines;

fn get_jolt(line: &str) -> u64 {
    let mut pos_list: Vec<usize> = vec![];
    let mut jolts: Vec<u32> = vec![];
    for ch in line.chars().into_iter() {
        let num = ch.to_digit(10).unwrap();
        jolts.push(num);
    }
    let count = 12;
    for l in 0..count {
        let split = jolts.len() - (count - l - 1);
        let start = match pos_list.last() {
            Some(x) => x + 1,
            _ => 0,
        };

        let max = *jolts[start..split].iter().max().unwrap();
        let pos = jolts[start..split]
            .iter()
            .enumerate()
            .find(|&(_, x)| *x == max)
            .map(|(i, _)| i)
            .unwrap()
            + start;
        pos_list.push(pos);
    }

    let mut value: u64 = 0;
    for x in pos_list {
        value = 10 * value + jolts[x] as u64;
    }
    value
}

pub fn solution() {
    let mut sum = 0;
    if let Ok(lines) = read_lines("./input/day3/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            let jolt = get_jolt(&line);
            sum += jolt;
            println!("bank {}: {} jolt {}", i, line, jolt);
        }
    }
    println!("sum {}", sum);
}
