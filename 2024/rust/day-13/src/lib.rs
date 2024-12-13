pub fn process_part1(input: &str) -> String {
    let mut ans = 0;

    for block in input.split("\n\n") {
        let lines = block.lines().collect::<Vec<&str>>();
        let (x1, y1) = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();

        let first = (
            x1.split_once("+").unwrap().1.parse::<i32>().unwrap(),
            y1.split_once("+").unwrap().1.parse::<i32>().unwrap(),
        );

        let (x2, y2) = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();

        let second = (
            x2.split_once("+").unwrap().1.parse::<i32>().unwrap(),
            y2.split_once("+").unwrap().1.parse::<i32>().unwrap(),
        );

        let prizes = lines[2]
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();

        let prizes = (
            prizes.0.split_once("=").unwrap().1.parse::<i32>().unwrap(),
            prizes.1.split_once("=").unwrap().1.parse::<i32>().unwrap(),
        );

        let y =
            (prizes.1 * first.0 - first.1 * prizes.0) / (second.1 * first.0 - first.1 * second.0);
        let x = (prizes.0 - second.0 * y) / first.0;

        if first.0 * x + second.0 * y == prizes.0 && first.1 * x + second.1 * y == prizes.1 {
            ans += x * 3 + y * 1;
        }
    }
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut ans = 0;

    for block in input.split("\n\n") {
        let lines = block.lines().collect::<Vec<&str>>();
        let (x1, y1) = lines[0]
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();

        let first = (
            x1.split_once("+").unwrap().1.parse::<i128>().unwrap(),
            y1.split_once("+").unwrap().1.parse::<i128>().unwrap(),
        );

        let (x2, y2) = lines[1]
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();

        let second = (
            x2.split_once("+").unwrap().1.parse::<i128>().unwrap(),
            y2.split_once("+").unwrap().1.parse::<i128>().unwrap(),
        );

        let prizes = lines[2]
            .split_once(": ")
            .unwrap()
            .1
            .split_once(", ")
            .unwrap();

        let prizes = (
            prizes.0.split_once("=").unwrap().1.parse::<i128>().unwrap() + 10000000000000,
            prizes.1.split_once("=").unwrap().1.parse::<i128>().unwrap() + 10000000000000,
        );

        let y =
            (prizes.1 * first.0 - first.1 * prizes.0) / (second.1 * first.0 - first.1 * second.0);
        let x = (prizes.0 - second.0 * y) / first.0;

        if first.0 * x + second.0 * y == prizes.0 && first.1 * x + second.1 * y == prizes.1 {
            ans += x * 3 + y * 1;
        }
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
        assert_eq!(result, "480");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "875318608908");
    }
}
