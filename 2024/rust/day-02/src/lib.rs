fn check(vec: Vec<i32>) -> bool {
    let mut increasing = false;
    if vec[0] < vec[1] {
        increasing = true;
    }
    let mut safe = true;
    for i in 0..vec.len() - 1 {
        if increasing {
            if vec[i + 1] - vec[i] > 3 || vec[i + 1] - vec[i] <= 0 {
                safe = false;
            }
        } else {
            if vec[i] - vec[i + 1] > 3 || vec[i] - vec[i + 1] <= 0 {
                safe = false;
            }
        }
    }
    safe
}

pub fn process_part1(input: &str) -> String {
    let mut ans = 0;
    input.lines().for_each(|line| {
        let vec = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        if check(vec) {
            ans += 1;
        }
    });
    // println!("{:?}", ans);
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut ans = 0;
    input.lines().for_each(|line| {
        let vec = line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect::<Vec<_>>();
        let mut safe = false;
        for (i, _) in vec.iter().enumerate() {
            let mut new_vec = vec.clone();
            new_vec.remove(i);
            if check(new_vec) {
                safe = true;
            }
        }
        if safe {
            ans += 1;
        }
    });
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
        assert_eq!(result, "2");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "4");
    }
}
