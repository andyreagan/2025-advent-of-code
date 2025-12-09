use std::fs::File;
use std::io::prelude::*;

fn area(p1: &Vec<i32>, p2: &Vec<i32>) -> f64 {
    let dx = (p1[0] - p2[0]).abs() as f64;
    let dy = (p1[1] - p2[1]).abs() as f64;
    dx*dy
}

fn process_day09(input: &str, connections_to_make: usize) -> (usize, i32) {
    let lines: Vec<&str> = input.lines().collect();
}

pub fn run(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let connections_to_make: usize = 10; // TODO: adjust for real input
    let (total, total_2) = process_day08(&input, connections_to_make);
    println!("total is {total}");
    println!("distance to the wall is {total_2}");
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_day09() {
        let input = "..............
.......#...#..
..............
..#....#......
..............
..#......#....
..............
.........#.#..
..............";
        let (total, total_2) = process_day09(input, 10);
        assert_eq!(total, 40);
        assert_eq!(total_2, 25272);
    }
}
