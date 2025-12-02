pub fn process_part1(input: &str) -> String {
    input
        .replace("\n", "")
        .split(',')
        .flat_map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse::<i64>().unwrap();
            let end = parts.next().unwrap().parse::<i64>().unwrap();
            start..=end
        })
        .filter(|&num| {
            let s = num.to_string();
            let len = s.len();
            if len % 2 != 0 {
                return false;
            }
            let half = len / 2;
            s[..half] == s[half..]
        })
        .sum::<i64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .replace("\n", "")
        .split(',')
        .flat_map(|range| {
            let mut parts = range.split('-');
            let start = parts.next().unwrap().parse::<i64>().unwrap();
            let end = parts.next().unwrap().parse::<i64>().unwrap();
            start..=end
        })
        .filter(|&num| {
            let s = num.to_string();
            let len = s.len();
            (1..=len / 2).any(|sub_len| {
                if len % sub_len != 0 {
                    return false;
                }
                let sub_str = &s.as_bytes()[0..sub_len];
                s.as_bytes().chunks(sub_len).all(|chunk| chunk == sub_str)
            })
        })
        .sum::<i64>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "1227775554");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "4174379265");
    }
}
