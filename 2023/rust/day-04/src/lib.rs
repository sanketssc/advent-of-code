pub fn process_part1(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<_>>()[1])
        .map(|card| card.split(" | ").collect::<Vec<_>>())
        .map(|card| {
            card.iter()
                .map(|side| {
                    side.split(" ")
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|val| {
                            if val.len() > 0 {
                                val.parse::<i32>().unwrap()
                            } else {
                                -1
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>()
        })
        .collect::<Vec<_>>();
    let mut sum = 0;

    for (_, val) in arr.iter().enumerate() {
        let mut mul = 0;
        for (_, v2) in val[0].iter().enumerate() {
            if *v2 != -1 {
                if val[1].contains(v2) {
                    if mul == 0 {
                        mul += 1;
                    } else {
                        mul *= 2;
                    }
                }
            }
        }
        sum += mul;
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<_>>()[1])
        .map(|card| card.split(" | ").collect::<Vec<_>>())
        .map(|card| {
            card.iter()
                .map(|side| {
                    side.split(" ")
                        .collect::<Vec<_>>()
                        .iter()
                        .map(|val| {
                            if val.len() > 0 {
                                val.parse::<i32>().unwrap()
                            } else {
                                -1
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<Vec<_>>>()
        })
        .collect::<Vec<_>>();
    // let mut sum = 0;
    let mut svec: Vec<i32> = Vec::new();
    // dbg!(arr.len());

    for _ in 0..arr.len() {
        svec.push(1);
    }
    // dbg!(svec);

    for (i, val) in arr.iter().enumerate() {
        for _ in 0..svec[i] {
            let mut mul = 0;
            for (_, v2) in val[0].iter().enumerate() {
                if *v2 != -1 {
                    if val[1].contains(v2) {
                        mul += 1;
                    }
                }
            }
            // dbg!(mul, svec.clone());

            for x in i + 1..=i + mul as usize {
                svec[x] += 1;
            }
        }
        // sum += mul;
    }

    let sum = svec.iter().sum::<i32>();
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
        assert_eq!(result, "13");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "30");
    }
}
