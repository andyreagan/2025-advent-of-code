use std::fs::File;
use std::io::prelude::*;

macro_rules! debug_println {
    ($($arg:tt)*) => {
        #[cfg(test)]
        println!($($arg)*);
    };
}

#[cfg(test)]
fn pretty_print_beam(beam: &Vec<i128>) -> String {
    beam.iter()
        .map(|x| {
            match x {
                1.. => '|',
                _ => '.',
            }
        }
        )
        .collect::<String>()
}

fn process_manifold(inputs: &[&str]) -> (i128, i128) {
    let height = inputs.len();
    // TODO: implement day 7 solution
    let width = inputs[0].len();
    println!("width is {width}");
    let mut beam: Vec<i128> = vec![0; width];
    beam[width/2] = 1;
    debug_println!("initialized beam {}", pretty_print_beam(&beam));    
    let mut split_count: i128 = 0;
    for i in 1..height {
        // try to continue existing beams
        debug_println!("processing input row {}", inputs[i]);
        debug_println!("starting beam {}", pretty_print_beam(&beam));
        let mut new_beam: Vec<i128> = beam.clone();
        // go across and look for splitters,
        // only matters if we have a beam to split
        for j in 0..width {
            // split it! 
            if beam[j] > 0 && inputs[i].chars().nth(j).expect("manifold") == '^' {
                debug_println!("found a split");
                new_beam[j-1] += beam[j];
                new_beam[j] = 0;
                new_beam[j+1] += beam[j];
                split_count += 1;
            }
        }
        beam = new_beam;
        debug_println!("ending beam {}", pretty_print_beam(&beam));
    }
    (split_count, beam.iter().sum())
}

pub fn run(day: i8) {
    println!("---------  day {}  ----------", day);
    let file = File::open(format!("inputs/day{:02}.txt", day));
    let mut input = String::new();
    let _ = file.expect(&format!("file inputs/day{:02}.txt does not exist", day)).read_to_string(&mut input);
    let inputs: Vec<&str> = input.lines().collect();
    println!("our tachyon is {} tall", inputs.len());
    let (total, total_2) = process_manifold(&inputs);
    println!("total is {total}");
    println!("total 2 is {total_2}");
    println!("------- end of day {} -------\n", day);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manifold() {
        let input = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
................";
        let inputs: Vec<&str> = input.lines().collect();
        let (total, total_2) = process_manifold(&inputs);
        assert_eq!(total, 21, "Day 7 total should be 21");
        assert_eq!(total_2, 40, "Day 7 part 2 total should be 40");
    }
}
