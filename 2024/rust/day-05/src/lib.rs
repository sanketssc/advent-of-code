use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let (block1, block2) = input.split_once("\n\n").unwrap();

    let mut mp: HashMap<&str, Vec<&str>> = HashMap::new();
    block1.lines().for_each(|line| {
        let (key, value) = line.split_once("|").unwrap();
        mp.entry(key).or_insert(vec![]).push(value);
    });
    let mut ans = 0;

    block2.lines().for_each(|line| {
        let vals = line.split(",").collect::<Vec<&str>>();
        let mut valid = true;
        for i in 0..vals.len() {
            let key = vals[i];
            for j in i + 1..vals.len() {
                let value = vals[j];
                if mp.contains_key(key) {
                    if !mp[key].contains(&value) {
                        valid = false;
                    }
                }

                if mp.contains_key(value) {
                    if mp[value].contains(&key) {
                        valid = false;
                    }
                }
            }
        }

        if valid {
            ans += vals[vals.len() / 2].parse::<i32>().unwrap();
        }
    });
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (block1, block2) = input.split_once("\n\n").unwrap();

    let mut mp: HashMap<&str, Vec<&str>> = HashMap::new();
    block1.lines().for_each(|line| {
        let (key, value) = line.split_once("|").unwrap();
        mp.entry(key).or_insert(vec![]).push(value);
    });
    let mut ans = 0;

    block2.lines().for_each(|line| {
        let vals = line.split(",").collect::<Vec<&str>>();
        let mut valid = false;
        for i in 0..vals.len() {
            let key = vals[i];
            for j in i + 1..vals.len() {
                let value = vals[j];
                if mp.contains_key(key) {
                    if !mp[key].contains(&value) {
                        valid = true;
                    }
                }

                if mp.contains_key(value) {
                    if mp[value].contains(&key) {
                        valid = true;
                    }
                }
            }
        }

        let mut temp = vec!["0"; vals.len()];
        if valid {
            for i in 0..vals.len() {
                let mut tcnt = 0;
                for j in 0..vals.len() {
                    if mp.contains_key(vals[i]) {
                        if mp[vals[i]].contains(&vals[j]) {
                            tcnt += 1;
                        }
                    }
                }
                temp[tcnt] = vals[i];
            }
        }

        if valid {
            ans += temp[temp.len() / 2].parse::<i32>().unwrap();
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
        assert_eq!(result, "143");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "123");
    }
}
