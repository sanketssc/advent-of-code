fn check(test_value: i128, numbers: &[i128]) -> bool {
    let operators = vec!['+', '*'];
    let mut stack = vec![(numbers[0], 1)];

    while let Some((current_value, index)) = stack.pop() {
        if index == numbers.len() {
            if current_value == test_value {
                return true;
            }
        } else {
            for &op in &operators {
                let next_value = match op {
                    '+' => current_value + numbers[index],
                    '*' => current_value * numbers[index],
                    _ => unreachable!(),
                };
                stack.push((next_value, index + 1));
            }
        }
    }

    false
}

pub fn process_part1(input: &str) -> String {
    let equations: Vec<(i128, Vec<i128>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let test_value = parts[0].parse().unwrap();
            let numbers = parts[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (test_value, numbers)
        })
        .collect();
    let sum: i128 = equations
        .into_iter()
        .filter_map(|(test_value, numbers)| {
            if check(test_value, &numbers) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum();
    sum.to_string()
}

fn check2(test_value: i128, numbers: &[i128]) -> bool {
    let operators = vec!["+", "*", "||"];
    let mut stack = vec![(numbers[0], 1)];

    while let Some((current_value, index)) = stack.pop() {
        if index == numbers.len() {
            if current_value == test_value {
                return true;
            }
        } else {
            for &op in &operators {
                let next_value = match op {
                    "+" => current_value + numbers[index],

                    "*" => current_value * numbers[index],
                    "||" => concatenate(current_value, numbers[index]),
                    _ => unreachable!(),
                };
                stack.push((next_value, index + 1));
            }
        }
    }

    false
}

fn concatenate(a: i128, b: i128) -> i128 {
    let mut b_digits = b;
    let mut multiplier = 1;
    while b_digits > 0 {
        multiplier *= 10;
        b_digits /= 10;
    }
    a * multiplier + b
}

pub fn process_part2(input: &str) -> String {
    let equations: Vec<(i128, Vec<i128>)> = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(": ").collect();
            let test_value = parts[0].parse().unwrap();
            let numbers = parts[1]
                .split_whitespace()
                .map(|num| num.parse().unwrap())
                .collect();
            (test_value, numbers)
        })
        .collect();
    let sum: i128 = equations
        .into_iter()
        .filter_map(|(test_value, numbers)| {
            if check2(test_value, &numbers) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum();
    sum.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "3749");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "11387");
    }
}
