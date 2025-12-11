use std::cmp::min;
use std::cmp::max;

pub fn process(input: &str) -> i128 {
    let split: Vec<&str> = input.split("\n\n").collect();
    let fresh_ranges: Vec<Vec<i128>> = split[0].lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(
            |x| {
                x.split("-")
                    .map(|n| n.parse::<i128>().expect("should parse to int"))
                    .collect()
            }
        )
        .collect();

    // part 2
    let mut starting_ranges = fresh_ranges.clone();
    starting_ranges.push(fresh_ranges[0].clone());
    let mut new_ranges: Vec<Vec<i128>> = fresh_ranges;
    while new_ranges.len() < starting_ranges.len() {
        starting_ranges = new_ranges;
        new_ranges = Vec::new();
        for new_range in starting_ranges.iter() {
            let mut combined: bool = false;
            for existing_range in new_ranges.iter_mut() {
                if new_range[0] <= existing_range[1] && new_range[1] >= existing_range[0]
                {
                    existing_range[0] = min(existing_range[0], new_range[0]);
                    existing_range[1] = max(existing_range[1], new_range[1]);
                    combined = true;
                    break;
                }
            }
            if !combined {
                new_ranges.push(new_range.to_vec());
            }
        }
    }
    new_ranges.iter().map(|x| x[1]-x[0]+1).sum()
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05b() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let new_ranges_total = process(input);
        assert_eq!(new_ranges_total, 14);
    }
}
