pub fn process_part1(input: &str) -> String {
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let ranges: Vec<(usize, usize)> = blocks[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();

    let count = blocks[1]
        .lines()
        .filter(|line| {
            let num: usize = line.parse().unwrap();
            ranges
                .iter()
                .any(|&(start, end)| num >= start && num <= end)
        })
        .count();

    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let blocks: Vec<&str> = input.split("\n\n").collect();

    let mut ranges: Vec<(usize, usize)> = blocks[0]
        .lines()
        .map(|line| {
            let mut parts = line.split('-');
            let start: usize = parts.next().unwrap().parse().unwrap();
            let end: usize = parts.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect();
    ranges.sort_unstable();

    let mut total: usize = 0;
    let mut current_start = ranges[0].0;
    let mut current_end = ranges[0].1;

    for &(start, end) in &ranges[1..] {
        if start <= current_end + 1 {
            current_end = current_end.max(end);
        } else {
            total += current_end - current_start + 1;
            current_start = start;
            current_end = end;
        }
    }
    total += current_end - current_start + 1;

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "3");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "6");
    }
}
