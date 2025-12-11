use std::cmp::max;

fn area(p1: &Vec<i32>, p2: &Vec<i32>) -> i64 {
    let dx = ((p1[0] - p2[0]).abs() + 1) as i64;
    let dy = ((p1[1] - p2[1]).abs() + 1) as i64;
    dx*dy
}

pub fn process(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();
    let points: Vec<Vec<i32>> = lines.iter()
        .map(|x| {
            x.split(",")
                .map(|n| n.parse::<i32>().expect("should parse to int"))
                .collect()
        })
        .collect();
    let mut biggest_area: i64 = 0;
    for i in 0..points.len() {
        for j in i..points.len() {
            biggest_area = max(biggest_area, area(&points[i], &points[j]));
        }
    }
    biggest_area
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day09a() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
        let biggest_area = process(input);
        assert_eq!(biggest_area, 50);
    }
}
