pub fn process(input: &str) -> i32 {
    let banks: Vec<&str> = input.lines().collect();
    let mut joltage_total: i32 = 0;
    for bank in banks.iter() {
        let mut max_joltage = 0;
        for i in 0..bank.len()-1 {
            for j in i+1..bank.len() {
                let substring = bank.chars().nth(i).expect("has value").to_string() + &bank.chars().nth(j).expect("has value").to_string();
                let joltage: i32 = substring.parse().unwrap();
                if joltage > max_joltage {
                    max_joltage = joltage
                }
            }
        }
        joltage_total += max_joltage;
    }
    joltage_total
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03a() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let joltage_total = process(input);
        assert_eq!(joltage_total, 357);
    }
}
