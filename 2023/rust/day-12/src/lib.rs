use std::collections::HashMap;

fn dp(
    str: &String,
    result_vec: &Vec<usize>,
    si: usize,
    ri: usize,
    cur: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(&result) = cache.get(&(si, ri, cur)) {
        return result;
    }

    if si == str.len() {
        if ri != result_vec.len() {
            return 0;
        }
        return 1;
    }
    let v = str.chars().nth(si).unwrap();
    let mut opts = vec![1, 0];
    if v == '#' {
        opts = vec![1];
    } else if v == '.' {
        opts = vec![0];
    }

    if ri == result_vec.len() {
        if opts.contains(&0) == false {
            return 0;
        }
        return dp(str, result_vec, si + 1, ri, 0, cache);
    }

    let mut ret = 0;
    if opts.contains(&0) {
        if cur == result_vec[ri] {
            ret += dp(&str, &result_vec, si + 1, ri + 1, 0, cache);
        }
        if cur == 0 {
            ret += dp(&str, &result_vec, si + 1, ri, 0, cache);
        }
    }
    if opts.contains(&1) {
        ret += dp(str, result_vec, si + 1, ri, cur + 1, cache)
    }

    cache.insert((si, ri, cur), ret);
    return ret;
}

pub fn process_part1(input: &str) -> String {
    let mut ans = 0;

    input.lines().for_each(|l| {
        let a = l.split(" ").nth(0).unwrap();
        let b = l
            .split(" ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        let x = dp(&(a.to_string() + "."), &b, 0, 0, 0, &mut HashMap::new());
        ans += x;
    });
    // println!("{:?}", arr);
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut ans = 0;
    let mut count = 0;
    input.lines().for_each(|l| {
        let old_a = l.split(" ").nth(0).unwrap();
        let old_b = l
            .split(" ")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let mut a: String = old_a.to_string().clone();
        let mut b = old_b.clone();
        for _ in 0..4 {
            a = a + "?" + old_a;
            b.append(&mut old_b.clone());
        }
        count = count + 1;
        // println!("row - {}, {} -------- {:?}", count, a, b);
        let x = dp(&(a + "."), &b, 0, 0, 0, &mut HashMap::new());
        ans += x;
    });
    // println!("{:?}", arr);
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
        assert_eq!(result, "21");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "525152");
    }
}
