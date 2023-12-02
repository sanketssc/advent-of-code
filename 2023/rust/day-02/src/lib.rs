pub fn process_part1(input: &str) -> String {
    let r = input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<_>>()[1])
        .map(|game| game.split("; ").collect::<Vec<_>>())
        .map(|gamep| {
            gamep
                .iter()
                .map(|g| g.split(", ").collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let a = r
        .iter()
        .map(|q| {
            let re = q
                .iter()
                .map(|g| {
                    let res = g
                        .iter()
                        .map(|gg| {
                            let x = gg.split(" ").collect::<Vec<_>>();
                            let a = match x[1] {
                                //12 red cubes, 13 green cubes, and 14 blue cubes?
                                "green" => {
                                    if x[0].parse::<i32>().unwrap() > 13 {
                                        return false;
                                    }
                                    true
                                }
                                "red" => {
                                    if x[0].parse::<i32>().unwrap() > 12 {
                                        return false;
                                    }
                                    true
                                }
                                "blue" => {
                                    if x[0].parse::<i32>().unwrap() > 14 {
                                        return false;
                                    }
                                    true
                                }
                                _ => false,
                            };
                            a
                        })
                        .collect::<Vec<_>>();

                    for i in res {
                        if !i {
                            return false;
                        }
                    }
                    true
                })
                .collect::<Vec<_>>();
            for i in re {
                if !i {
                    return false;
                }
            }
            true
        })
        .collect::<Vec<_>>();
    let mut sum = 0;
    for (i, &val) in a.iter().enumerate() {
        if val {
            sum += i + 1;
        }
    }
    println!("{:?}", sum);
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let r = input
        .lines()
        .map(|line| line.split(": ").collect::<Vec<_>>()[1])
        .map(|game| game.split("; ").collect::<Vec<_>>())
        .map(|gamep| {
            gamep
                .iter()
                .map(|g| g.split(", ").collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let a = r
        .iter()
        .map(|q| {
            let re = q
                .iter()
                .map(|g| {
                    let res = g
                        .iter()
                        .map(|gg| {
                            let x = gg.split(" ").collect::<Vec<_>>();
                            let mut v = vec![0, 0, 0];
                            let _a = match x[1] {
                                //12 red cubes, 13 green cubes, and 14 blue cubes?
                                "green" => {
                                    v[1] = x[0].parse::<i32>().unwrap();
                                }
                                "red" => {
                                    v[2] = x[0].parse::<i32>().unwrap();
                                }
                                "blue" => {
                                    v[0] = x[0].parse::<i32>().unwrap();
                                }
                                _ => {}
                            };
                            // a
                            // x[0].parse::<i32>().unwrap()
                            v
                        })
                        .collect::<Vec<_>>();
                    // println!("{:?}", res);
                    let mut v = vec![0, 0, 0];
                    for i in res {
                        v[0] += i[0];
                        v[1] += i[1];
                        v[2] += i[2];
                    }
                    // println!("{:?}", v);

                    v
                })
                .collect::<Vec<_>>();
            let mut v = vec![0, 0, 0];
            for i in re {
                if i[0] > v[0] {
                    v[0] = i[0];
                }
                if i[1] > v[1] {
                    v[1] = i[1];
                }
                if i[2] > v[2] {
                    v[2] = i[2];
                }
            }
            // println!("{:?}", v);
            v[0] * v[1] * v[2]
        })
        .sum::<i32>();

    a.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "8");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "2286");
    }
}
