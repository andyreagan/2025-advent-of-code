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

pub fn process(input: &str) -> i128 {
    let inputs: Vec<&str> = input.lines().collect();
    let height = inputs.len();
    let width = inputs[0].len();
    debug_println!("width is {width}");
    let mut beam: Vec<i128> = vec![0; width];
    beam[width/2] = 1;
    debug_println!("initialized beam {}", pretty_print_beam(&beam));
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
            }
        }
        beam = new_beam;
        debug_println!("ending beam {}", pretty_print_beam(&beam));
    }
    beam.iter().sum()
}

pub fn main() {
    let input = include_str!("../input.txt");
    println!("{}", process(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day07b() {
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
        let total = process(input);
        assert_eq!(total, 40);
    }
}
