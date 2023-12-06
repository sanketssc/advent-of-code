pub fn process_part1(input: &str) -> String {
    let arr = input
        .split("\r\n")
        .map(|val| {
            val.split(": ")
                .nth(1)
                .map(|x| {
                    x.trim()
                        .split_whitespace()
                        .map(|each| each.trim().parse::<i32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .unwrap()
        })
        .collect::<Vec<_>>();
    let mut prod = 1;
    for i in 0..arr[0].len() {
        let mut count = 0;
        let mut j = 1;
        let mut end = arr[0][i] - 1;
        while j < arr[0][i] {
            if j * end > arr[1][i] {
                count += 1;
            }
            j += 1;
            end -= 1;
        }

        prod *= count;
    }

    prod.to_string()
}

pub fn process_part2(input: &str) -> String {
    let arr = input
        .split("\r\n")
        .map(|val| {
            val.split(": ")
                .nth(1)
                .map(|x| {
                    x.trim()
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join("")
                        .parse::<i64>()
                        .unwrap()
                })
                .unwrap()
        })
        .collect::<Vec<_>>();

    let mut count = 0;
    let mut j = 1;
    let mut end = arr[0] - 1;
    while j < arr[0] {
        if j * end > arr[1] {
            count += 1;
        }
        j += 1;
        end -= 1;
    }
    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "288");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "71503");
    }
}
