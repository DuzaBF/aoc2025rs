use crate::read_lines::read_lines;

fn update_line(first: &Vec<char>, second: &mut Vec<char>, sum: &mut usize) {
    let mut pass: Vec<usize> = vec![];
    let mut split: Vec<usize> = vec![];
    {
        for (i, ch) in second.iter().enumerate() {
            if first[i] == '|' || first[i] == 'S' {
                if *ch == '^' {
                    split.push(i);
                } else {
                    pass.push(i);
                }
            }
        }
    }

    *sum = *sum + split.len();
    for pos in split {
        if pos > 0 {
            if second[pos - 1] != '^' {
                second[pos - 1] = '|'
            }
        }
        if pos < second.len() - 1 {
            if second[pos + 1] != '^' {
                second[pos + 1] = '|'
            }
        }
    }

    for pos in pass {
        second[pos] = '|';
    }
}

fn print(grid: &Vec<Vec<char>>) {
    for line in grid {
        println!("{line:?}");
    }
}

pub fn solution() {
    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines("./input/day7/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            grid.push(line.chars().into_iter().collect());
            // println!("{:?}", grid[i]);
        }
    }

    for i in 0..grid.len() - 1 {
        // println!("---- step {i} ----");
        // print(&grid);
        let pair = grid.split_at_mut(i + 1);
        update_line(&pair.0[i], &mut pair.1[0], &mut sum);
    }

    println!("---- final ----");
    print(&grid);
    println!("sum {}", sum);
}
