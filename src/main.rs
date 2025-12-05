use std::fs::File;
use std::io::prelude::*;
use std::cmp::min;
use std::cmp::max;
// use num_bigint::BigUint;
use more_asserts::assert_ge;

fn day01() {
    // println!("Hello, world!");
    println!("---------  day 1  ----------");    
    // println!("{} {}", -32 % 100, -32 / 100);
    let input = String::from("L68\nL30\nR48\nL5\nR60\nL55\nL101\nL99\nR14\nL82");
    // let file = File::open("inputs/day01.txt");
    // let mut input = String::new();
    // let _ = file.expect("file input.txt does not exist").read_to_string(&mut input);
    let turns: Vec<&str> = input.lines().collect();
    println!("we have to make {} turns", turns.len());
    let mut n_zeros = 0;
    let mut full_turns = 0;
    let mut position: i16 = 50;
    // for i in 0..turns.len() {    
    //     println!("{}", turns[i])
    // }
    for turn in turns.iter() {
        let direction = turn.chars().nth(0).expect("Need to have a direction");
        let clicks: String = turn.chars().skip(1).collect();
        let click_count: i16 = clicks.parse().unwrap();
        // let starting_position = position;
        // let starting_zeros = n_zeros;
        // let starting_full_turns = full_turns;
        match direction {
            'L' => position -= click_count % 100,
            'R' => position += click_count % 100,
            _ => panic!(),
        }
        // get the full rotations in the turn itself
        match direction {
            'L' => full_turns += click_count / 100,
            'R' => full_turns += click_count / 100,
            _ => panic!(),
        }
        // passed zero to the right:
        if position > 100 {
            full_turns += 1
        }
        position = position % 100;
        // passed zero to the left
        if position < 0 {
            // but didn't start at zero!
            if position != (-1 * (click_count % 100)) {
               full_turns += 1            
            }
            position += 100;
        }
        if position == 0 {
            n_zeros += 1
        }
        // println!("starting {} move {} to new position {}, landed on zero {} times, passed zero = {} times", starting_position, turn, position, n_zeros - starting_zeros, full_turns - starting_full_turns);
    }
    println!("we hit zero {} times", n_zeros);
    assert_eq!(n_zeros, 3);
    println!("we crossed zero {} times for a total of {} zeros", full_turns, full_turns + n_zeros);
    assert_eq!(full_turns + n_zeros, 7);
    println!("------- end of day 1 -------\n");
}

fn day02() {
    println!("---------  day 2  ----------");
    let input = String::from("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124");
    // let file = File::open("inputs/day02.txt");
    // let mut input = String::new();
    // let _ = file.expect("file input.txt does not exist").read_to_string(&mut input);
    let ranges: Vec<&str> = input.split(',').collect();
    println!("we have to check {} ranges", ranges.len());
    let mut match_count = 0;
    let mut match_total: u64 = 0;
    let mut match_count_all = 0;
    let mut match_total_all: u64 = 0;    
    for range in ranges.iter() {
        // println!("range = {}", range);
        let range_parts: Vec<&str> = range.split('-').collect();
        let start: u64 = range_parts[0].to_string().parse().unwrap();
        let end: u64 = range_parts[1].to_string().parse().unwrap();
        for i in start..end+1 {
            // println!("{}", i);
            let i_str = i.to_string();
            // let i_chars = i_str.chars().collect();
            let str_len = i_str.len();
            // println!("string has length {}", str_len);
            if str_len % 2 == 0 {
               let first_half = &i_str[..str_len/2];
               let second_half = &i_str[str_len/2..];
               // println!("string is even, has parts {} and {}, match = {}", first_half, second_half, first_half == second_half);
               if first_half == second_half {
                   // println!("{} is even, has parts {} and {}, match = {}", i_str, first_half, second_half, first_half == second_half);
                   match_count += 1;
                   match_total += i;
               }
            }
            for j in 1..str_len {
               if str_len % j == 0 {
                   let substr = &i_str[..j];
                   let repeats = str_len / j;
                   if substr.repeat(repeats) == i_str {
                       // println!("{} is a repeat of substr {}", i_str, substr);                   
                       match_count_all += 1;
                       match_total_all += i;
                       break;
                   }
               }
            }
            
        } 
    }
    println!("match count = {}, match_total = {}", match_count, match_total);
    assert_eq!(match_count, 8);
    assert_eq!(match_total, 1227775554);
    println!("match count_all = {}, match_total_all = {}", match_count_all, match_total_all);
    assert_eq!(match_count_all, 13);
    assert_eq!(match_total_all, 4174379265);
    println!("------- end of day 2 -------\n");
}


fn day03(day: i32) {
    println!("---------  day {}  ----------", day);
    let input = String::from("987654321111111\n811111111111119\n234234234234278\n818181911112111");
    // let file = File::open(format!("inputs/day{:02}.txt", day));
    // let mut input = String::new();
    // let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let banks: Vec<&str> = input.lines().collect();
    println!("we have to check {} battery banks", banks.len());
    let mut joltage_total: i32 = 0;
    // let mut joltage_total_2 = BigUint::ZERO;
    let mut joltage_total_2: i128 = 0;
    for bank in banks.iter() {
        let mut max_joltage = 0;
        // println!("bank = {}, has len {}", bank, bank.len());
        for i in 0..bank.len()-1 {
            // println!("{}", i);
            // let i_str = i.to_string();
            // let i_chars = i_str.chars().collect();
            // let str_len = i_str.len();
            // println!("string has length {}", str_len);
            for j in i+1..bank.len() {
                // println!("[{},{}] -> {}{}", i, j, bank.chars().nth(i).expect("has value"), bank.chars().nth(j).expect("has value"));
                let substring = bank.chars().nth(i).expect("has value").to_string() + &bank.chars().nth(j).expect("has value").to_string();
                let joltage: i32 = substring.parse().unwrap();
                if joltage > max_joltage {
                    max_joltage = joltage
                }
            }
        }
        joltage_total += max_joltage;
        // part 2        
        // we'll let i, j, k be the batteries of the 15 that we're not including
        // which splits our 15 batteries in 4 slices
        // let mut max_joltage_2 = BigUint::ZERO;
        // for i in 0..bank.len()-2 {
        //     for j in i+1..bank.len()-1 {
        //         for k in j+1..bank.len() {
        //             let mut batteries = String::new();
        //             // println!("{i}-{j}-{k}");
        //             // section 1, between 0 and i
        //             if i > 0 {
        //                 // println!("section 1 has chars {}", &bank[..i]);
        //                 batteries.push_str(&bank[..i]);
        //             }
        //             // section 2, between i and j
        //             if i + 1 < j {
        //                 // println!("section 2 has chars {}", &bank[i+1..j]);
        //                 batteries.push_str(&bank[i+1..j]);
        //             }
        //             // section 3, between j and k
        //             if j + 1 < k {
        //                 // println!("section 3 has chars {}", &bank[j+1..k]);
        //                 batteries.push_str(&bank[j+1..k]);
        //             }
        //             // section 4, between k and the end
        //             if k < bank.len()-1 {
        //                 // println!("section 4 has chars {}", &bank[k+1..]);
        //                 batteries.push_str(&bank[k+1..]);
        //             }                    
        //             // assert_eq!(batteries.len(), 12);
        //             let joltage: BigUint = batteries.parse().unwrap();
        //             if joltage > max_joltage_2 {
        //                 // println!("new max joltage = {joltage}");
        //                 max_joltage_2 = joltage;
        //             }
        //         }
        //     }
        // }
        // joltage_total_2 += max_joltage_2;
        // part 2 redux
        // that only worked when we had banks of 15, with 15-12=3 to _not use_.
        // instead, we need to focus on using 12 batteries
        // we'll start at the beginning of the bank and find the biggest battery
        // moving right, leaving 11 places at the end
        // save that biggest battery to the bank and it's position
        // then start where we left off and go until 10 places left
        // etc
        // consider converting the bank to a vector of ints
        // so we don't have to convert so many times!
        let mut battery = String::new();
        let mut position_max: usize = 0;
        let mut value_str = bank.chars().nth(position_max).expect("has position");
        // let mut position_max: i32 = 0;
        // let mut value_str = &bank[position_max..position_max+1];
        let mut value_max = value_str as i32;
        // println!("got initial value of {value_str}");
        for cell in 0..12 {
            // println!("checking for cell {cell}");
            for i in position_max+1..bank.len()-(12-cell)+1 {
                // println!("checking position {i} for a higher value");
                value_str = bank.chars().nth(i).expect("has position");
                let value_tmp = value_str as i32;
                if value_tmp > value_max {
                    // println!("we got a new max position");
                    position_max = i;
                    value_max = value_tmp;
                }
            }
            battery.push(bank.chars().nth(position_max).expect("has position"));
            // println!("battery is now {battery}");
            // for the next cell, we'll start it at the next position
            // with that value as it's current max
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
    println!("joltage_total = {}", joltage_total);
    assert_eq!(joltage_total, 357);
    println!("joltage_total_2 = {}", joltage_total_2);
    assert_eq!(joltage_total_2, 3121910778619);
    println!("------- end of day {} -------\n", day);
}


fn in_bounds(x: &usize, y: &usize, i: &usize, j: &usize) -> bool {
    // println!("checking in bounds at position {i}, {j}");
    *i >= 1 && *i < x+1 && *j >= 1 && *j < y+1
}

fn safe_access(arr: &Vec<Vec<bool>>, x: &usize, y: &usize, i: &usize, j: &usize) -> i8 {
    if in_bounds(x, y, i, j) {
        // println!("in bounds, return actual value {}", arr[*i-1][*j-1]);
        return arr[*i-1][*j-1] as i8
    }
    // println!("out of bounds");
    0
}


fn day04(day: i32) {
    println!("---------  day {}  ----------", day);
    let input = String::from("..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.");
    // let file = File::open(format!("inputs/day{:02}.txt", day));
    // let mut input = String::new();
    // let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let lines: Vec<&str> = input.lines().collect();
    let mut arr: Vec<Vec<bool>> = Vec::new();
    // could pad the sides, so I don't have to do so much checking on the size
    for line in lines.iter() {
        let line_vec: Vec<bool> = line.chars().map(|x| x == '@').collect();
        arr.push(line_vec);
    }
    let size = (arr.len(), arr[0].len());
    println!("array size {}x{}", size.0, size.1);
    let mut reachable = 0;
    for i in 1..size.0+1 {
        for j in 1..size.1+1 {
            
            if arr[i-1][j-1] {
                // println!("roll here, check position {i},{j}");
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
                    // println!("REACHABLE !!!!!!!!!!");
                    // print!("x");
                    reachable += 1;

                } else {
                    // print!("@");
                }
            } else {
                // print!(".");
            }
        }
        // println!(" ");
    }
    println!("reachable = {reachable} on first scan, not removing any");
    assert_eq!(reachable, 13);
    // restart a scan while we change it live!
    reachable = 0;
    let mut reachable_this_scan = 0;
    for i in 1..size.0+1 {
        for j in 1..size.1+1 {
            
            if arr[i-1][j-1] {
                // println!("roll here, check position {i},{j}");
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
                    // println!("REACHABLE !!!!!!!!!!");
                    // print!("x");
                    reachable += 1;
                    reachable_this_scan += 1;
                    arr[i-1][j-1] = false;
                } else {
                    // print!("@");
                }
            } else {
                // print!(".");
            }
        }
        // println!(" ");
    }
    println!("reached {reachable} on first scan with changes, will certainly rescan");
    assert_ge!(reachable, 13);
    assert_ge!(reachable_this_scan, 1);    
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
                        // print!("x");
                        reachable += 1;
                        reachable_this_scan += 1;
                        arr[i-1][j-1] = false;
                    } else {
                        // print!("@");
                    }
                } else {
                    // print!(".");
                }
            }
            // println!(" ");
        }    
        if reachable_this_scan > 0 {
            println!("reached {reachable_this_scan} this round, rescanning...");
        } else {
            println!("reached 0, done!");
        }
    }
    assert_eq!(reachable, 43);
    println!("reachable = {reachable} after all scans");
    println!("------- end of day {} -------\n", day);
}


fn day05(day: i32) {
    println!("---------  day {}  ----------", day);
    let input = String::from("3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32");
    // let file = File::open(format!("inputs/day{:02}.txt", day));
    // let mut input = String::new();
    // let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input); 
    let split: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(split.len(), 2);
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
    // for the test data:
    println!("we have {} id ranges", fresh_ranges.len());
    println!("we have {} ids", food_ids.len());
    // assert_eq!(fresh_ranges.len(), 4);
    // assert_eq!(food_ids.len(), 6);

    let mut fresh_ingredients: i128 = 0;
    for food_id in food_ids.into_iter() {
        // println!("checking food {food_id}");
        for fresh_range in fresh_ranges.iter() {
            if food_id >= fresh_range[0] && food_id <= fresh_range[1] {
                // println!("found in range {}-{}", fresh_range[0], fresh_range[1]);
                fresh_ingredients += 1;
                break;
            } // else {
            //     println!("not in range {}-{}", fresh_range[0], fresh_range[1]);
            // }
        }
    }
    // assert_eq!(fresh_ingredients, 3);
    println!("fresh = {fresh_ingredients}");
    // part 2
    let mut starting_ranges = fresh_ranges.clone();
    // add an duplicate to get the while loop going
    starting_ranges.push(fresh_ranges[0].clone());
    let mut new_ranges: Vec<Vec<i128>> = fresh_ranges;
    while new_ranges.len() < starting_ranges.len() {
        println!("iterating again...with {} starting ranges", new_ranges.len());
        // clear the new ranges
        starting_ranges = new_ranges;
        new_ranges = Vec::new();
        for new_range in starting_ranges.iter() {
            // check overlap with any existing in new_range
            // combine into first overlap
            let mut combined: bool = false;
            for existing_range in new_ranges.iter_mut() {
                if new_range[0] <= existing_range[1] && new_range[1] >= existing_range[0]
                {
                    // combine them
                    println!("combining {}-{} into {}-{}", new_range[0], new_range[1], existing_range[0], existing_range[1]);
                    existing_range[0] = min(existing_range[0], new_range[0]);
                    existing_range[1] = max(existing_range[1], new_range[1]);
                    println!("new range is {}-{}", existing_range[0], existing_range[1]);
                    combined = true;
                    break;
                }
            }
            // if no overlap, push onto the array
            if !combined {
                println!("appending range {}-{}", new_range[0], new_range[1]);
                new_ranges.push(new_range.to_vec());
            }
        }
        println!("finished with {} ranges", new_ranges.len());
    }
    let new_ranges_total: i128 = new_ranges.iter().map(|x| x[1]-x[0]+1).sum();
    // assert_eq!(new_ranges_total, 14);
    println!("total valid ids are {new_ranges_total}");
    println!("------- end of day {} -------\n", day);
}


fn main() {
    day01();
    day02();
    day03(3);
    day04(4);
    day05(5);    
}
