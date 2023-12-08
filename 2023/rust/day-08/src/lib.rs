use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Direction {
    left: String,
    right: String,
}

fn get_map(data: Vec<&str>) -> HashMap<String, Direction> {
    let mut hm: HashMap<String, Direction> = HashMap::new();
    data.iter().for_each(|val| {
        let x = val.split(" = ").collect::<Vec<_>>();
        let word = &x[1][1..9].split(", ").collect::<Vec<_>>();
        let left = word[0];
        let right = word[1];
        let dir = Direction {
            left: left.to_string(),
            right: right.to_string(),
        };
        hm.insert(x[0].to_string(), dir);
    });
    hm
}

pub fn process_part1(input: &str) -> String {
    let instructions = input.lines().nth(0).unwrap();
    let data = input
        .split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();
    let mut start = "AAA".to_string();
    let mp = get_map(data);
    let mut x = 0;
    let mut count = 0;

    while start != "ZZZ".to_string() {
        let d = instructions.chars().nth(x).unwrap();
        match d {
            'L' => start = mp.get(&start).unwrap().left.clone(),
            'R' => start = mp.get(&start).unwrap().right.clone(),
            _ => (),
        }
        println!("{:?} ----- {:?} ----- {}", start, d, count);
        count += 1;
        x = (x + 1) % instructions.len();
    }

    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let instructions = input.lines().nth(0).unwrap();
    let data = input
        .split("\r\n\r\n")
        .nth(1)
        .unwrap()
        .lines()
        .collect::<Vec<_>>();

    let vals_arr = data
        .iter()
        .map(|val| val.split(" = ").nth(0).unwrap().to_string())
        .collect::<Vec<_>>();

    let mut a_arr: Vec<String> = Vec::new();
    vals_arr.iter().for_each(|val| {
        if val.chars().last().unwrap() == 'A' {
            a_arr.push(val.to_string());
        }
    });
    let mp = get_map(data);
    let mut x = 0;
    let mut counts: Vec<i128> = Vec::new();
    for i in 0..a_arr.len() {
        let mut count = 0;

        while a_arr[i].chars().last().unwrap() != 'Z' {
            let d = instructions.chars().nth(x).unwrap();

            match d {
                'L' => a_arr[i] = mp.get(&a_arr[i]).unwrap().left.clone(),
                'R' => a_arr[i] = mp.get(&a_arr[i]).unwrap().right.clone(),
                _ => (),
            }

            count += 1;
            x = (x + 1) % instructions.len();
        }
        counts.push(count);
    }
    let mut lcm = counts[0];
    for i in 1..counts.len() {
        for j in 1..=counts[i] {
            if lcm * j as i128 % counts[i as usize] == 0 {
                lcm = lcm * j as i128;
                break;
            }
        }
    }

    lcm.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "6");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "6");
    }
}
