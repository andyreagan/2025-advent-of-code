use std::fs::File;
use std::io::prelude::*;

fn area(p1: &Vec<i32>, p2: &Vec<i32>) -> f64 {
    let dx = (p1[0] - p2[0]).abs() as f64;
    let dy = (p1[1] - p2[1]).abs() as f64;
    dx*dy
}

fn process_day09(input: &str) -> (i64, i64) {
    let lines: Vec<&str> = input.lines().collect();
    (0, 0)
}

pub fn run(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (part_1, part_2) = process_day09(&input);
    println!("biggest rectangle is {part_1}");
    println!("TBD is {part_2}");
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_day09() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let (part_1, part_2) = process_day09(input);
        assert_eq!(part_1, 40);
        assert_eq!(part_2, 0);
    }
}
