use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];
    for line in input.lines() {
        let v: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        v1.push(v[0]);
        v2.push(v[1]);
    }
    v1.sort();
    v2.sort();

    let mut ans = 0;

    for i in 0..v1.len() {
        ans += (v2[i] - v1[i]).abs();
    }

    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut v1: Vec<i32> = vec![];
    let mut v2: Vec<i32> = vec![];
    for line in input.lines() {
        let v: Vec<i32> = line
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        v1.push(v[0]);
        v2.push(v[1]);
    }

    let mut ans = 0;

    let mut frequency2: HashMap<i32, i32> = HashMap::new();

    for i in v2 {
        *frequency2.entry(i).or_insert(0) += 1;
    }

    v1.sort();

    for i in v1 {
        ans += i * frequency2.get(&i).unwrap_or(&0);
    }

    ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "11");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "31");
    }
}
