
pub fn process_part1(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 50;
    let mut pass = 0;
    for line in lines {
        let first = line.chars().next().unwrap();
        let number: i32 = line[1..].trim().parse().unwrap();
        match first {
            'L' => sum -= number,
            'R' => sum += number,
            _ => panic!("Invalid direction"),
        }
        sum = (sum + 100) % 100;
        if sum == 0 {
            pass += 1;
        }
    }
   pass.to_string()
}

pub fn process_part2(input: &str) -> String {
    let lines = input.lines();
    let mut sum = 50;
    let mut pass = 0;
    let mut prevsumzero = false;
    for line in lines {
        let first = line.chars().next().unwrap();
        let mut number: i32 = line[1..].trim().parse().unwrap();
        pass += number / 100;
        number %= 100;
        match first {
            'L' => sum -= number,
            'R' => sum += number,
            _ => panic!("Invalid direction"),
        }
        if (sum < 0 || sum > 100) && !prevsumzero {
            pass += 1;
        }
        sum = (sum + 100) % 100;
        if sum == 0 {
            pass += 1;
            prevsumzero = true;
        }else {
            prevsumzero = false;
        }
    }
   pass.to_string()
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
