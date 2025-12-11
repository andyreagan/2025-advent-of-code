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
    let food_ids: Vec<i128> = split[1].lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|n| n.parse::<i128>().expect("should parse to int"))
        .collect();

    let mut fresh_ingredients: i128 = 0;
    for food_id in food_ids.into_iter() {
        for fresh_range in fresh_ranges.iter() {
            if food_id >= fresh_range[0] && food_id <= fresh_range[1] {
                fresh_ingredients += 1;
                break;
            }
        }
    }
    fresh_ingredients
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day05a() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let fresh_ingredients = process(input);
        assert_eq!(fresh_ingredients, 3);
    }
}
