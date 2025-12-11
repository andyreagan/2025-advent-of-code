pub fn process(input: &str) -> i128 {
    let inputs: Vec<&str> = input.lines().collect();
    let height = inputs.len();
    let width = inputs[0].len();
    let mut beam: Vec<i128> = vec![0; width];
    beam[width/2] = 1;
    for i in 1..height {
        let mut new_beam: Vec<i128> = beam.clone();
        for j in 0..width {
            if beam[j] > 0 && inputs[i].chars().nth(j).expect("manifold") == '^' {
                new_beam[j-1] += beam[j];
                new_beam[j] = 0;
                new_beam[j+1] += beam[j];
            }
        }
        beam = new_beam;
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
