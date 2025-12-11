pub fn process(input: &str) -> (u64, u64) {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut match_count = 0;
    let mut match_total: u64 = 0;
    for range in ranges.iter() {
        let range_parts: Vec<&str> = range.split('-').collect();
        let start: u64 = range_parts[0].to_string().parse().unwrap();
        let end: u64 = range_parts[1].to_string().parse().unwrap();
        for i in start..=end {
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
        }
    }
    (match_count, match_total)
}

pub fn main() {
    let input = include_str!("../input.txt");
    let (_, match_total) = process(input);
    println!("{}", match_total);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02a() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let (match_count, match_total) = process(input);
        assert_eq!(match_count, 8);
        assert_eq!(match_total, 1227775554);
    }
}
