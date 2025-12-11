pub fn process(input: &str) -> i16 {
    let turns: Vec<&str> = input.lines().collect();
    let mut n_zeros = 0;
    let mut full_turns = 0;
    let mut position: i16 = 50;
    for turn in turns.iter() {
        let direction = turn.chars().nth(0).expect("Need to have a direction");
        let clicks: String = turn.chars().skip(1).collect();
        let click_count: i16 = clicks.parse().unwrap();
        match direction {
            'L' => position -= click_count % 100,
            'R' => position += click_count % 100,
            _ => panic!(),
        }
        match direction {
            'L' => full_turns += click_count / 100,
            'R' => full_turns += click_count / 100,
            _ => panic!(),
        }
        if position > 100 {
            full_turns += 1
        }
        position = position % 100;
        if position < 0 {
            if position != (-1 * (click_count % 100)) {
               full_turns += 1
            }
            position += 100;
        }
        if position == 0 {
            n_zeros += 1
        }
    }
    full_turns + n_zeros
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day01b() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL101\nL99\nR14\nL82";
        let total = process(input);
        assert_eq!(total, 7);
    }
}
