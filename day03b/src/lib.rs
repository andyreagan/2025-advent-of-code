pub fn process(input: &str) -> i128 {
    let banks: Vec<&str> = input.lines().collect();
    let mut joltage_total_2: i128 = 0;
    for bank in banks.iter() {
        let mut battery = String::new();
        let mut position_max: usize = 0;
        let mut value_str = bank.chars().nth(position_max).expect("has position");
        let mut value_max = value_str as i32;
        for cell in 0..12 {
            for i in position_max+1..=bank.len()-(12-cell) {
                value_str = bank.chars().nth(i).expect("has position");
                let value_tmp = value_str as i32;
                if value_tmp > value_max {
                    position_max = i;
                    value_max = value_tmp;
                }
            }
            battery.push(bank.chars().nth(position_max).expect("has position"));
            position_max += 1;
            if cell < 11 {
                value_str = bank.chars().nth(position_max).expect("has position");
                let value_tmp = value_str as i32;
                value_max = value_tmp;
            }
        }
        let joltage: i128 = battery.parse().unwrap();
        joltage_total_2 += joltage;
    }
    joltage_total_2
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day03b() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let joltage_total_2 = process(input);
        assert_eq!(joltage_total_2, 3121910778619);
    }
}
