fn in_bounds(x: &usize, y: &usize, i: &usize, j: &usize) -> bool {
    *i >= 1 && *i < x+1 && *j >= 1 && *j < y+1
}

fn safe_access(arr: &Vec<Vec<bool>>, x: &usize, y: &usize, i: &usize, j: &usize) -> i8 {
    if in_bounds(x, y, i, j) {
        return arr[*i-1][*j-1] as i8
    }
    0
}

pub fn process(input: &str) -> i32 {
    let lines: Vec<&str> = input.lines().collect();
    let mut arr: Vec<Vec<bool>> = Vec::new();
    for line in lines.iter() {
        let line_vec: Vec<bool> = line.chars().map(|x| x == '@').collect();
        arr.push(line_vec);
    }
    let size = (arr.len(), arr[0].len());
    let mut reachable = 0;
    let mut reachable_this_scan = 1;
    while reachable_this_scan > 0 {
        reachable_this_scan = 0;
        for i in 1..=size.0 {
            for j in 1..=size.1 {
                if arr[i-1][j-1] {
                    if (
                        safe_access(&arr, &size.0, &size.1, &(i-1), &(j-1)) +
                            safe_access(&arr, &size.0, &size.1, &(i-1), &(j)) +
                            safe_access(&arr, &size.0, &size.1, &(i-1), &(j+1)) +
                            safe_access(&arr, &size.0, &size.1, &(i), &(j-1)) +
                            safe_access(&arr, &size.0, &size.1, &(i), &(j+1)) +
                            safe_access(&arr, &size.0, &size.1, &(i+1), &(j-1)) +
                            safe_access(&arr, &size.0, &size.1, &(i+1), &(j)) +
                            safe_access(&arr, &size.0, &size.1, &(i+1), &(j+1))
                    ) < 4 {
                        reachable += 1;
                        reachable_this_scan += 1;
                        arr[i-1][j-1] = false;
                    }
                }
            }
        }
    }
    reachable
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    use more_asserts::assert_ge;

    #[test]
    fn test_day04b() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let reachable = process(input);
        assert_ge!(reachable, 13);
        assert_eq!(reachable, 43);
    }
}
