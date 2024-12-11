use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let mut nums = input
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    for _ in 0..25 {
        let mut temp = vec![];
        for &i in nums.iter() {
            let number_of_digits = i.to_string().len();
            if i == 0 {
                temp.push(1);
            } else if number_of_digits % 2 == 0 {
                let first_half = i.to_string()[0..number_of_digits / 2]
                    .parse::<u128>()
                    .unwrap();
                let second_half = i.to_string()[number_of_digits / 2..]
                    .parse::<u128>()
                    .unwrap();
                temp.push(first_half);
                temp.push(second_half);
            } else {
                temp.push(i * 2024);
            }
        }
        nums = temp;
    }

    nums.len().to_string()
}

pub fn process_part2(input: &str) -> String {
    let num = input
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();
    let mut nums: HashMap<u128, u128> = HashMap::new();
    for &i in num.iter() {
        nums.insert(i, 1);
    }

    for _ in 0..75 {
        let mut temp = HashMap::new();
        for (&k, &v) in nums.iter() {
            let number_of_digits = k.to_string().len();
            if k == 0 {
                temp.entry(1).and_modify(|e| *e += v).or_insert(v);
            } else if number_of_digits % 2 == 0 {
                let first_half = k.to_string()[0..number_of_digits / 2]
                    .parse::<u128>()
                    .unwrap();
                let second_half = k.to_string()[number_of_digits / 2..]
                    .parse::<u128>()
                    .unwrap();
                temp.entry(first_half).and_modify(|e| *e += v).or_insert(v);
                temp.entry(second_half).and_modify(|e| *e += v).or_insert(v);
            } else {
                temp.entry(k * 2024).and_modify(|e| *e += v).or_insert(v);
            }
        }
        nums = temp;
    }

    nums.values().sum::<u128>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "55312");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "65601038650482");
    }
}
