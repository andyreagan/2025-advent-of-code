pub fn process(input: &str) -> (u64, u64) {
    let ranges: Vec<&str> = input.split(',').collect();
    let mut match_count_all = 0;
    let mut match_total_all: u64 = 0;
    for range in ranges.iter() {
        let range_parts: Vec<&str> = range.split('-').collect();
        let start: u64 = range_parts[0].to_string().parse().unwrap();
        let end: u64 = range_parts[1].to_string().parse().unwrap();
        for i in start..=end {
            let i_str = i.to_string();
            let str_len = i_str.len();
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
    (match_count_all, match_total_all)
}

pub fn main() {
    let input = include_str!("../input.txt");
    let (_, match_total_all) = process(input);
    println!("{}", match_total_all);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day02b() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        let (match_count_all, match_total_all) = process(input);
        assert_eq!(match_count_all, 13);
        assert_eq!(match_total_all, 4174379265);
    }
}
