use crate::read_lines::read_lines;

type Point = (usize, usize);

fn draw(reds: &Vec<Point>) {
    let x_size = reds.iter().max_by(|a, b| a.0.cmp(&b.0)).unwrap().0;
    let y_size = reds.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap().1;

    for y in 0..y_size {
        for x in 0..x_size + 1 {
            if reds.iter().find(|&&value| value == (x, y)).is_some() {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn area(corners: (Point, Point)) -> usize {
    let x = corners.0.0.abs_diff(corners.1.0) + 1;
    let y = corners.0.1.abs_diff(corners.1.1) + 1;
    x * y
}

fn find_max_area(reds: &Vec<Point>) -> (Point, Point) {
    let mut corners: (Point, Point) = ((0, 0), (0, 0));
    let mut max_area = 0;

    for i in 0..reds.len() {
        for j in i..reds.len() {
            let area = area((reds[i], reds[j]));
            if area > max_area {
                max_area = area;
                corners = (reds[i], reds[j]);
            }
        }
    }

    corners
}

pub fn solution() {
    let mut reds: Vec<Point> = vec![];
    if let Ok(lines) = read_lines("./input/day9/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            let values: Vec<&str> = line.split(",").collect();
            let mut p: Point = (0, 0);
            p.0 = values[0].parse().unwrap();
            p.1 = values[1].parse().unwrap();
            reds.push(p);
        }
    }

    // draw(&reds);
    let corners = find_max_area(&reds);
    println!("{:?} = {:?}", corners, area(corners));
}
