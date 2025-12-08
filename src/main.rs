// use std::fs::File;
// use std::io::prelude::*;
use std::cmp::min;
use std::cmp::max;
// use num_bigint::BigUint;
#[cfg(test)]
use more_asserts::assert_ge;

fn process_day01(input: &str) -> (i16, i16) {
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
    (n_zeros, full_turns + n_zeros)
}

fn run_day01(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (n_zeros, total) = process_day01(&input);
    println!("we hit zero {} times", n_zeros);
    println!("total zeros (hit + crossed) = {}", total);
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests_day01 {
    use super::*;

    #[test]
    fn test_day01() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL101\nL99\nR14\nL82";
        let (n_zeros, total) = process_day01(input);
        assert_eq!(n_zeros, 3);
        assert_eq!(total, 7);
    }
}

fn process_day02(input: &str) -> (u64, u64, u64, u64) {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut match_count = 0;
    let mut match_total: u64 = 0;
    let mut match_count_all = 0;
    let mut match_total_all: u64 = 0;
    for range in ranges.iter() {
        let range_parts: Vec<&str> = range.split('-').collect();
        let start: u64 = range_parts[0].to_string().parse().unwrap();
        let end: u64 = range_parts[1].to_string().parse().unwrap();
        for i in start..end+1 {
            let i_str = i.to_string();
            let str_len = i_str.len();
            if str_len % 2 == 0 {
               let first_half = &i_str[..str_len/2];
               let second_half = &i_str[str_len/2..];
               if first_half == second_half {
                   match_count += 1;
                   match_total += i;
               }
            }
            for j in 1..str_len {
               if str_len % j == 0 {
                   let substr = &i_str[..j];
                   let repeats = str_len / j;
                   if substr.repeat(repeats) == i_str {
                       match_count_all += 1;
                       match_total_all += i;
                       break;
                   }
               }
            }
        }
    }
    (match_count, match_total, match_count_all, match_total_all)
}

fn run_day02(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (match_count, match_total, match_count_all, match_total_all) = process_day02(&input);
    println!("match count = {}, match_total = {}", match_count, match_total);
    println!("match count_all = {}, match_total_all = {}", match_count_all, match_total_all);
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests_day02 {
    use super::*;

    #[test]
    fn test_day02() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let (match_count, match_total, match_count_all, match_total_all) = process_day02(input);
        assert_eq!(match_count, 8);
        assert_eq!(match_total, 1227775554);
        assert_eq!(match_count_all, 13);
        assert_eq!(match_total_all, 4174379265);
    }
}

fn process_day03(input: &str) -> (i32, i128) {
    let banks: Vec<&str> = input.lines().collect();
    let mut joltage_total: i32 = 0;
    let mut joltage_total_2: i128 = 0;
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
        let mut battery = String::new();
        let mut position_max: usize = 0;
        let mut value_str = bank.chars().nth(position_max).expect("has position");
        let mut value_max = value_str as i32;
        for cell in 0..12 {
            for i in position_max+1..bank.len()-(12-cell)+1 {
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
    (joltage_total, joltage_total_2)
}

fn run_day03(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (joltage_total, joltage_total_2) = process_day03(&input);
    println!("joltage_total = {}", joltage_total);
    println!("joltage_total_2 = {}", joltage_total_2);
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests_day03 {
    use super::*;

    #[test]
    fn test_day03() {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111";
        let (joltage_total, joltage_total_2) = process_day03(input);
        assert_eq!(joltage_total, 357);
        assert_eq!(joltage_total_2, 3121910778619);
    }
}

fn in_bounds(x: &usize, y: &usize, i: &usize, j: &usize) -> bool {
    *i >= 1 && *i < x+1 && *j >= 1 && *j < y+1
}

fn safe_access(arr: &Vec<Vec<bool>>, x: &usize, y: &usize, i: &usize, j: &usize) -> i8 {
    if in_bounds(x, y, i, j) {
        return arr[*i-1][*j-1] as i8
    }
    0
}

fn process_day04(input: &str) -> (i32, i32) {
    let lines: Vec<&str> = input.lines().collect();
    let mut arr: Vec<Vec<bool>> = Vec::new();
    for line in lines.iter() {
        let line_vec: Vec<bool> = line.chars().map(|x| x == '@').collect();
        arr.push(line_vec);
    }
    let size = (arr.len(), arr[0].len());
    let mut reachable_first = 0;
    for i in 1..size.0+1 {
        for j in 1..size.1+1 {
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
                    reachable_first += 1;
                }
            }
        }
    }
    let mut reachable = 0;
    let mut reachable_this_scan = 1;
    while reachable_this_scan > 0 {
        reachable_this_scan = 0;
        for i in 1..size.0+1 {
            for j in 1..size.1+1 {
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
    (reachable_first, reachable)
}

fn run_day04(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (reachable_first, reachable) = process_day04(&input);
    println!("reachable = {} on first scan", reachable_first);
    println!("reachable = {} after all scans", reachable);
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests_day04 {
    use super::*;

    #[test]
    fn test_day04() {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        let (reachable_first, reachable) = process_day04(input);
        assert_eq!(reachable_first, 13);
        assert_ge!(reachable, 13);
        assert_eq!(reachable, 43);
    }
}

fn process_day05(input: &str) -> (i128, i128) {
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
    let new_ranges_total: i128 = new_ranges.iter().map(|x| x[1]-x[0]+1).sum();
    (fresh_ingredients, new_ranges_total)
}

fn run_day05(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (fresh_ingredients, new_ranges_total) = process_day05(&input);
    println!("fresh = {}", fresh_ingredients);
    println!("total valid ids = {}", new_ranges_total);
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests_day05 {
    use super::*;

    #[test]
    fn test_day05() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (fresh_ingredients, new_ranges_total) = process_day05(input);
        assert_eq!(fresh_ingredients, 3);
        assert_eq!(new_ranges_total, 14);
    }
}

fn process_day06(input: &str) -> (i128, i128) {
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
    // part 2
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
    (total, total_2)
}

fn run_day06(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let (total, total_2) = process_day06(&input);
    println!("total = {}", total);
    println!("total_2 = {}", total_2);
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests_day06 {
    use super::*;

    #[test]
    fn test_day06() {
        let input = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";
        let (total, total_2) = process_day06(input);
        assert_eq!(total, 4277556);
        assert_eq!(total_2, 3263827);
    }
}


fn distance(p1: &Vec<i32>, p2: &Vec<i32>) -> f64 {
    let dx = (p1[0] - p2[0]).abs() as f64;
    let dy = (p1[1] - p2[1]).abs() as f64;
    let dz = (p1[2] - p2[2]).abs() as f64;
    (dx*dx + dy*dy + dz*dz).sqrt()
}

fn day08(day: i32) {
    println!("---------  day {}  ----------", day);
    let connections_to_make: usize = 10;
    let input = String::from("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689");
    // let file = File::open(format!("inputs/day{:02}.txt", day));
    // let mut input = String::new();
    // let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let lines: Vec<&str> = input.lines().collect();
    println!("we have {} boxes to check", lines.len());
    let points: Vec<Vec<i32>> = lines.iter()
        .map(|x| {
            x.split(",")
                .map(|n| n.parse::<i32>().expect("should parse to int"))
                .collect()
        })
        .collect();
    println!("compute the full pairwise distance matrix");
    // let mut distances: Vec<Vec<f64>> = vec![vec![0.0; points.len()]; points.len()];
    // let mut connections: Vec<Vec<i32>> = vec![vec![0; points.len()]; points.len()];
    // this is a more sortable format for the matrix (a sparse representation)
    let mut all_distances: Vec<(usize, usize, f64)> = Vec::new();
    for i in 0..points.len() {
        // only need to compute half the matrix
        // (and not the diagonal)
        for j in i+1..points.len() {
            let distance = distance(&points[i], &points[j]);
            // distances[i][j] = distance;
            all_distances.push((i, j, distance));
        }
    }
    println!("sort all of the distances");
    println!("the first row of our distance list is like: {:?}", all_distances[0]);
    all_distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    println!("the first row of our sorted distance list is like: {:?}", all_distances[0]);
    // println!("this means the shortest distance is between box {:?} and box {:?}, with distance {}", points[all_distances[0].0], points[all_distances[0].1], all_distances[0].2);
    // println!("second is between box {:?} and box {:?}, with distance {}", points[all_distances[1].0], points[all_distances[1].1], all_distances[1].2);
    // now let's build up the connected components list
    let mut components: Vec<Vec<usize>> = Vec::new(); // vec![0..points.len()]
    // initialize every one as separate
    for i in 0..points.len() {
        components.push(vec![i]);
    }
    for i in 0..connections_to_make {
        let connection: (usize, usize, f64) = all_distances[i];
        // println!("connecting points {} and {}", connection.0, connection.1);
        let mut new_component: Vec<usize> = Vec::new();
        let mut other_components: Vec<Vec<usize>> = Vec::new();
        for component in components.iter() {
            // if either side of our connection is in the component,
            // we'll pull these into the new_component to merge and stick it on the end
            // otherwise, we'll push it onto the list
            if component.contains(&connection.0) || component.contains(&connection.1) {
                // println!("this component has one vertex of our new edge, it's in our _new_ component");
                for vertex in component.iter() {
                    new_component.push(*vertex);
                }
            } else {
                // println!("this component has no vertex on our new edge, so it won't grow");
                other_components.push(component.clone());
            }
        }
        // now rebuild the components
        components = other_components;
        components.push(new_component);
        // println!("after connection {}, we have {} components", i+1, components.len());
    }
    // now let's get the size of the components, sort that list, and get the largest 3
    let mut component_sizes: Vec<usize> = components.iter().map(|x| x.len()).collect();
    component_sizes.sort();
    component_sizes.reverse();
    println!("the largest 3 components have sizes: {}, {}, {}", component_sizes[0], component_sizes[1], component_sizes[2]);
    let total: usize = component_sizes[0] * component_sizes[1] * component_sizes[2];
    assert_eq!(total, 40);
    println!("total is {total}");

    // now let's keep going until the thing is fully connected
    let mut fully_connected: bool = components.len() == 1;
    let mut connections_made = connections_to_make;
    while !fully_connected {
        let connection: (usize, usize, f64) = all_distances[connections_made];
        let mut new_component: Vec<usize> = Vec::new();
        let mut other_components: Vec<Vec<usize>> = Vec::new();
        for component in components.iter() {
            if component.contains(&connection.0) || component.contains(&connection.1) {
                for vertex in component.iter() {
                    new_component.push(*vertex);
                }
            } else {
                other_components.push(component.clone());
            }
        }
        components = other_components;
        components.push(new_component);

        connections_made += 1;
        fully_connected = components.len() == 1;
    }
    connections_made -= 1;
    println!("it took {connections_made} connections to fully connect it");
    println!("final connection between {:?} and {:?}", points[all_distances[connections_made].0], points[all_distances[connections_made].1]);
    let total_2 = points[all_distances[connections_made].0][0] * points[all_distances[connections_made].1][0];
    assert_eq!(total_2, 25272);
    println!("distance to the wall is {total_2}");
    println!("------- end of day {} -------\n", day);
}

mod day07;    

fn main() {
    day01();
    day02();
    day03(3);
    day04(4);
    day05(5);
    day06(6);
    day07::run(7);    
    day08(8);        
}
