pub fn process(input: &str) -> i128 {
    let mut inputs: Vec<&str> = input.lines().collect();
    let signs: &str = inputs.pop().expect("have a final line");
    let signs_parsed: Vec<&str> = signs.split_whitespace().collect();
    let inputs_parsed: Vec<Vec<i128>> = inputs.iter()
        .map(|x| {
            x.split_whitespace()
                .map(|n| {
                    n.parse()
                        .expect("should be int")
                }).collect()
        }).collect();
    let mut pivoted: Vec<Vec<i128>> = Vec::new();
    for i in 0..inputs_parsed[0].len() {
        let mut tmp_vec: Vec<i128> = Vec::new();
        for j in 0..inputs_parsed.len() {
            tmp_vec.push(inputs_parsed[j][i]);
        }
        pivoted.push(tmp_vec);
    }
    let mut total: i128 = 0;
    for i in 0..pivoted.len() {
        let tmp_op: i128 = match signs_parsed[i] {
            "*" => pivoted[i].iter().product(),
            "+" => pivoted[i].iter().sum(),
            _ => panic!(),
        };
        total += tmp_op;
    }
    total
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06a() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let total = process(input);
        assert_eq!(total, 4277556);
    }
}
