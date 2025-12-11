pub fn process(input: &str) -> i128 {
    let mut inputs: Vec<&str> = input.lines().collect();
    let signs: &str = inputs.pop().expect("have a final line");
    let signs_parsed: Vec<&str> = signs.split_whitespace().collect();

    // part 2: parse based on column widths
    let mut widths: Vec<usize> = Vec::new();
    let mut counter: usize = 0;
    for i in 1..signs.len() {
        match signs.chars().nth(i).expect("have a char") {
            ' ' => counter += 1,
            _ => {
                widths.push(counter);
                counter = 0;
            },
        }
    }
    widths.push(counter+1);

    let mut inputs_parsed_2: Vec<Vec<&str>> = Vec::new();
    for line in inputs {
        let mut tmp_vec: Vec<&str> = Vec::new();
        let mut total_width: usize = 0;
        for width in widths.iter() {
            tmp_vec.push(&line[total_width..total_width+width]);
            total_width += width + 1;
        }
        inputs_parsed_2.push(tmp_vec);
    }
    let mut pivoted_2: Vec<Vec<&str>> = Vec::new();
    for i in 0..inputs_parsed_2[0].len() {
        let mut tmp_vec: Vec<&str> = Vec::new();
        for j in 0..inputs_parsed_2.len() {
            tmp_vec.push(inputs_parsed_2[j][i]);
        }
        pivoted_2.push(tmp_vec);
    }
    let mut inputs_parsed_3: Vec<Vec<i128>> = Vec::new();
    for i in 0..pivoted_2.len() {
        let width = widths[i];
        let mut tmp_vec: Vec<i128> = Vec::new();
        for j in 0..width {
            let collected_chars = pivoted_2[i].iter().map(|x| x.chars().nth(j).expect("have a char")).collect::<String>();
            tmp_vec.push(collected_chars.trim().parse().unwrap());
        }
        inputs_parsed_3.push(tmp_vec);
    }
    let mut total_2: i128 = 0;
    for i in 0..inputs_parsed_3.len() {
        let tmp_op: i128 = match signs_parsed[i] {
            "*" => inputs_parsed_3[i].iter().product(),
            "+" => inputs_parsed_3[i].iter().sum(),
            _ => panic!(),
        };
        total_2 += tmp_op;
    }
    total_2
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day06b() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let total_2 = process(input);
        assert_eq!(total_2, 3263827);
    }
}
