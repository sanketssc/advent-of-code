pub fn process_part1(input: &str) -> String {
    println!("{:?}", input);
    input.to_string()
}

pub fn process_part2(input: &str) -> String {
    println!("{:?}", input);
    input.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "ans");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "ans2");
    }
}
