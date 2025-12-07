use crate::read_lines::read_lines;

pub fn solution() {
    let mut nums: Vec<Vec<u64>> = vec![];
    let mut ops: Vec<String> = vec![];
    if let Ok(lines) = read_lines("./input/day6/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            nums.push(vec![]);
            for word in line.split(" ").filter(|&x| x != "") {
                match word.parse::<u64>() {
                    Ok(x) => nums[i].push(x),
                    __ => ops.push(word.to_string()),
                }
            }
        }
    }

    let mut columns: Vec<u64> = vec![0; ops.len()];

    for j in 0..ops.len() {
        if ops[j] == "*" {
            columns[j] = 1;
        }

        for i in 0..nums.len() - 1 {
            if ops[j] == "*" {
                columns[j] = columns[j] * nums[i][j];
            }

            if ops[j] == "+" {
                columns[j] = columns[j] + nums[i][j];
            }
        }
    }

    let sum: u64 = columns.into_iter().sum();
    println! {"sum {sum}"};
}
