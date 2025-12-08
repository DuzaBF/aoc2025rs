use crate::read_lines::read_lines;

pub fn solution() {
    let mut nums: Vec<Vec<u64>> = vec![];
    let mut ops: Vec<(char, usize)> = vec![];
    let mut rev_lines: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines("./input/day6/input") {
        for (_, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            rev_lines.push(line.chars().rev().collect());
        }
    }
    println!("{rev_lines:?}");

    {
        let mut l = 1;
        for ch in rev_lines.last().unwrap_or(&vec![]) {
            l += 1;
            if *ch == '*' || *ch == '+' {
                ops.push((*ch, l - 1));
                l = 0;
            }
        }
    }

    for el in &ops {
        nums.push(vec![0; el.1]);
    }

    println!("{:?}", ops);
    println!("{:?}", nums);

    for (_, line) in rev_lines[..rev_lines.len() - 1].iter().enumerate() {
        let mut operation = 0;
        let mut operand = 0_i32;
        for (_, ch) in line.iter().enumerate() {
            if operand == ops[operation].1 as i32 {
                operation += 1;
                operand = -1;
            }
            if operand >= 0 {
                let value = ch.to_digit(10);
                if let Some(x) = value {
                    nums[operation][operand as usize] *= 10;
                    nums[operation][operand as usize] += x as u64;
                }
            }
            operand += 1;
        }
    }

    println!("{:?}", nums);

    let mut columns: Vec<u64> = vec![0; ops.len()];
    for j in 0..ops.len() {
        if ops[j].0 == '*' {
            columns[j] = 1;
        }

        for i in 0..nums[j].len() {
            if ops[j].0 == '*' {
                columns[j] = columns[j] * nums[j][i];
            }

            if ops[j].0 == '+' {
                columns[j] = columns[j] + nums[j][i];
            }
        }
    }
    let sum: u64 = columns.into_iter().sum();
    println! {"sum {sum}"};
}
