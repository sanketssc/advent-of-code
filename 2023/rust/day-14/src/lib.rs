use std::collections::HashMap;

pub fn process_part1(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| {
            let mut l = line.split("").collect::<Vec<_>>();
            l.remove(0);
            l.remove(l.len() - 1);
            l
        })
        .collect::<Vec<_>>();
    let length = arr.len();

    let mut weight = 0;
    for j in 0..arr[0].len() {
        let mut last_hash_idx = 0;
        for i in 0..length {
            if arr[i][j] == "#" {
                last_hash_idx = i + 1;
            }
            if arr[i][j] == "O" {
                weight += length - last_hash_idx;
                last_hash_idx += 1;
            }
        }
    }
    println!("{}", weight);
    weight.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut arr = input
        .lines()
        .map(|line| {
            let mut l = line.split("").collect::<Vec<_>>();
            l.remove(0);
            l.remove(l.len() - 1);
            l
        })
        .collect::<Vec<_>>();
    let length = arr.len();

    let mut val: HashMap<usize, usize> = HashMap::new();
    let mut seen: HashMap<String, usize> = HashMap::new();
    let mut ans = 0;
    for t in 0..1000000000 {
        // println!("{}", loop_count);
        //1
        for j in 0..arr[0].len() {
            let mut last_hash_idx = 0;
            for i in 0..length {
                if arr[i][j] == "#" {
                    last_hash_idx = i + 1;
                }
                if arr[i][j] == "O" {
                    arr[i][j] = ".";
                    arr[last_hash_idx][j] = "O";
                    last_hash_idx += 1;
                }
            }
        }

        //2
        for i in 0..length {
            let mut last_hash_idx = 0;
            for j in 0..arr[0].len() {
                if arr[i][j] == "#" {
                    last_hash_idx = j + 1;
                }
                if arr[i][j] == "O" {
                    arr[i][j] = ".";
                    arr[i][last_hash_idx] = "O";
                    last_hash_idx += 1;
                }
            }
        }

        //3
        for j in 0..arr[0].len() {
            let mut last_hash_idx = length - 1;
            for i in (0..length).rev() {
                // println!("{} {}", i, last_hash_idx);
                if arr[i][j] == "#" {
                    if i > 0 {
                        last_hash_idx = i - 1;
                    }
                }
                if arr[i][j] == "O" {
                    arr[i][j] = ".";
                    arr[last_hash_idx][j] = "O";
                    if last_hash_idx > 0 {
                        last_hash_idx -= 1;
                    }
                }
            }
        }

        //4
        for i in 0..length {
            let mut last_hash_idx = arr[0].len() - 1;
            for j in (0..arr[0].len()).rev() {
                if arr[i][j] == "#" {
                    if j > 0 {
                        last_hash_idx = j - 1;
                    }
                }
                if arr[i][j] == "O" {
                    arr[i][j] = ".";
                    arr[i][last_hash_idx] = "O";
                    if last_hash_idx > 0 {
                        last_hash_idx -= 1;
                    }
                }
            }
        }
        // for j in 0..arr[0].len() {
        //     let mut last_hash_idx = 0;
        //     for i in 0..length {
        //         if arr[i][j] == "#" {
        //             last_hash_idx = i + 1;
        //         }
        //         if arr[i][j] == "O" {
        //             weight += length - last_hash_idx;
        //             last_hash_idx += 1;
        //         }
        //     }
        // }
        // println!("{}", weight);

        let mut tot = 0;
        for i in 0..length {
            for j in 0..arr[0].len() {
                if arr[i][j] == "O" {
                    tot += length - i;
                }
            }
        }
        val.insert(t, tot);
        let mut s = String::new();

        for r in arr.iter() {
            s += &r.join("");
        }

        if !seen.contains_key(&s) {
            seen.insert(s, t);
        } else {
            let per = t - seen.get(&s).unwrap();
            let x = t - per + (1000000000 - t - 1) % per;
            ans = *val.get(&x).unwrap();
            println!("{}", seen.len());
            break;
        }
    }
    // println!("{}", ans);
    // for i in arr.iter() {
    //     let tmp = String::from(i.join(""));
    //     println!("{}", tmp);
    // }

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
        assert_eq!(result, "136");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "64");
    }
}
