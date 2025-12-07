use core::num;
use std::fmt::Display;

use crate::read_lines::read_lines;

fn convert(grid: &Vec<Vec<char>>) -> Vec<Vec<i64>> {
    let mut num_grid = vec![vec![]; grid.len()];
    for i in 0..grid.len() {
        num_grid[i].resize(grid[i].len(), 0);
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                num_grid[i][j] = 1;
            }
            if grid[i][j] == '^' {
                num_grid[i][j] = -1;
            }
        }
    }
    num_grid
}

fn update_line(first: &Vec<i64>, second: &mut Vec<i64>, sum: &mut usize) {
    let mut pass: Vec<(usize, i64)> = vec![];
    let mut split: Vec<(usize, i64)> = vec![];

    for (i, ch) in second.iter().enumerate() {
        if first[i] > 0 {
            if *ch == -1 {
                split.push((i, first[i]));
            } else {
                pass.push((i, first[i]));
            }
        }
    }

    for (pos, val) in pass {
        second[pos] += val;
    }

    *sum = *sum + split.len();
    for (pos, val) in split {
        if pos > 0 {
            if second[pos - 1] != -1 {
                second[pos - 1] += val;
            }
        }
        if pos < second.len() - 1 {
            if second[pos + 1] != -1 {
                second[pos + 1] += val;
            }
        }
    }
}

fn print<T: Display>(grid: &Vec<Vec<T>>) {
    for line in grid {
        for ch in line {
            print!("{ch:4}")
        }
        println!();
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

    let mut num_grid = convert(&grid);
    println!("---- num ----");
    print(&num_grid);
    for i in 0..grid.len() - 1 {
        // println!("---- step {i} ----");
        // print(&grid);
        let pair = num_grid.split_at_mut(i + 1);
        update_line(&pair.0[i], &mut pair.1[0], &mut sum);
    }

    println!("---- final ----");
    print(&num_grid);
    println!("sum {}", sum);
    println!("timelines {}", num_grid.last().unwrap().iter().sum::<i64>());
}
