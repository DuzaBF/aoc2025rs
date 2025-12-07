use crate::read_lines::read_lines;

fn get_neighbors(i: usize, j: usize, i_len: usize, j_len: usize) -> Vec<(usize, usize)> {
    let dirs: [(isize, isize); 8] = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];
    let mut n: Vec<(usize, usize)> = vec![];
    for d in dirs {
        let n_i = i as isize + d.0;
        let n_j = j as isize + d.1;
        if n_i < i_len as isize && n_i >= 0 && n_j < j_len as isize && n_j >= 0 {
            n.push((n_i as usize, n_j as usize));
        }
    }
    n
}

fn find_removable(grid: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut removable: Vec<(usize, usize)> = vec![];
    for (i, line) in grid.iter().enumerate() {
        for (j, el) in line.iter().enumerate() {
            let mut count: u32 = 0;
            if *el == '@' {
                for neighbor in get_neighbors(i, j, grid.len(), line.len()) {
                    if grid[neighbor.0][neighbor.1] == '@' {
                        count += 1;
                    }
                }
                if count < 4 {
                    removable.push((i, j));
                }
            }
        }
    }
    removable
}

fn update_grid(grid: &mut Vec<Vec<char>>, removable: &Vec<(usize, usize)>) {
    for el in removable {
        grid[el.0][el.1] = '.';
    }
}

pub fn solution() {
    let mut sum = 0;
    let mut grid: Vec<Vec<char>> = vec![];
    if let Ok(lines) = read_lines("./input/day4/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            grid.push(vec![]);
            for ch in line.chars().into_iter() {
                grid[i].push(ch);
            }
            // println!("{:?}", grid[i]);
        }
    }

    let mut removable = find_removable(&grid);
    let mut i = 0;
    while removable.len() > 0 {
        sum += removable.len();
        update_grid(&mut grid, &removable);
        // println!("-- step {i} --");
        for line in &grid {
            // println!("{:?}", line)
        }
        removable = find_removable(&grid);
        i += 1;
    }

    println!("sum {}", sum);
}
