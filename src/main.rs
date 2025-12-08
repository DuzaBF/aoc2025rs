pub mod day1;
pub mod day1p2;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day7;
pub mod day8;
pub mod read_lines;

fn main() {
    use std::time::Instant;
    let now = Instant::now();
    day8::solution();
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
