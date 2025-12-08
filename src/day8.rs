use std::{fmt::Display, mem::swap};

use crate::read_lines::read_lines;

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    id: usize,
}

#[derive(Debug)]
struct Circuits {
    points: Vec<Point>,
    connections: Vec<Vec<usize>>,
}

#[derive(Debug)]
struct PairPoints {
    pair: (usize, usize),
}

#[derive(Debug)]
struct PairCircuits {
    pair: (usize, usize),
}

impl Display for Circuits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = Ok(());
        for (i, conn) in self.connections.iter().enumerate() {
            let _ = write!(f, "circuit #{:3}: [", i);
            for &l in conn {
                let _ = write!(
                    f,
                    "{:3}-({:3}, {:3}, {:3}), ",
                    self.points[l].id, self.points[l].x, self.points[l].y, self.points[l].z
                );
            }
            let _ = writeln!(f, "]");
        }

        res
    }
}

fn distance2(first: &Point, second: &Point) -> i64 {
    (first.x - second.x).pow(2) + (first.y - second.y).pow(2) + (first.z - second.z).pow(2)
}

fn find_next_closest(circuits: &Circuits, last_dist: i64) -> (i64, PairPoints, PairCircuits) {
    let mut min = i64::MAX;
    let mut pair_points: PairPoints = PairPoints { pair: (0, 0) };

    for i in 0..circuits.points.len() {
        for j in i + 1..circuits.points.len() {
            let dist = distance2(&circuits.points[i], &circuits.points[j]);
            if dist < min && dist > last_dist {
                min = dist;
                pair_points.pair = (circuits.points[i].id, circuits.points[j].id);
            }
        }
    }

    let mut first = 0;
    let mut second = 0;

    for (i, circuit) in circuits.connections.iter().enumerate() {
        for &id in circuit {
            if id == pair_points.pair.0 {
                first = i;
            }
            if id == pair_points.pair.1 {
                second = i;
            }
        }
    }

    if first >= second {
        swap(&mut first, &mut second);
    }
    let pair_circuits: PairCircuits = PairCircuits {
        pair: (first, second),
    };
    (min, pair_points, pair_circuits)
}

fn merge(circuits: &mut Circuits, pair_circuits: PairCircuits) -> bool {
    if pair_circuits.pair.0 == pair_circuits.pair.1 {
        return false;
    }
    let first = pair_circuits.pair.0;
    let second = pair_circuits.pair.1;
    for i in 0..circuits.connections[second].len() {
        let (left, right) = circuits.connections.split_at_mut(second);
        left[first].push(right[0][i]);
    }
    circuits.connections.remove(second);
    true
}

fn get_value(circuits: &Circuits) -> usize {
    let mut lengths: Vec<usize> = vec![0; circuits.connections.len()];
    for (i, circuit) in circuits.connections.iter().enumerate() {
        lengths[i] = circuit.len();
    }
    lengths.sort_by(|a, b| b.cmp(a));
    let mut result = 1;
    for i in 0..3 {
        if i < lengths.len() {
            result *= lengths[i]
        }
    }
    result
}

pub fn solution() {
    let mut circuits: Circuits = Circuits {
        points: vec![],
        connections: vec![],
    };
    if let Ok(lines) = read_lines("./input/day8/input") {
        for (i, line) in lines.map_while(Result::ok).into_iter().enumerate() {
            let values: Vec<&str> = line.split(",").collect();
            let mut p: Point = Point {
                x: 0,
                y: 0,
                z: 0,
                id: 0,
            };
            p.x = values[0].parse().unwrap();
            p.y = values[1].parse().unwrap();
            p.z = values[2].parse().unwrap();
            p.id = i;
            circuits.points.push(p);
            circuits.connections.push(vec![]);
            circuits.connections[i].push(i);
        }
    }
    println!("{circuits}");

    let mut dist: i64 = 0;

    let mut merges = 0;
    let mut pair_points: PairPoints = PairPoints { pair: (0, 0) };
    let mut pair_circuits: PairCircuits = PairCircuits { pair: (0, 0) };
    while circuits.connections.len() > 1 {
        (dist, pair_points, pair_circuits) = find_next_closest(&circuits, dist);

        // println!(
        //     "dist {} merge id {:?} circuits {:?}",
        //     dist, pair_points.pair, pair_circuits.pair
        // );

        merge(&mut circuits, pair_circuits);

        merges += 1;
        // println!("----{merges}----");
        // println!("{circuits}");
    }

    println!(
        "{}",
        circuits.points[pair_points.pair.0].x * circuits.points[pair_points.pair.1].x
    );
}
