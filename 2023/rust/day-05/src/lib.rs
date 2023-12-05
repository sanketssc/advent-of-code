use rayon::prelude::*;
use std::sync::Mutex;

fn get_vals(ip: &str) -> Vec<Vec<i64>> {
    let val = ip.split(":\r\n").collect::<Vec<_>>()[1]
        .split("\r\n")
        .map(|row| {
            row.split(" ")
                .map(|val| val.parse::<i64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    val
}

pub fn process_part1(input: &str) -> String {
    let arr = input.split("\r\n\r\n").collect::<Vec<_>>();
    let seeds = arr[0].split(": ").collect::<Vec<_>>()[1]
        .split(" ")
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut ans: Vec<i64> = Vec::new();

    for mut seed in seeds {
        for i in 1..arr.len() {
            let mut new_seed = seed;
            let a2 = get_vals(arr[i]);
            for row in a2 {
                if (row[1]..(row[1] + row[2])).contains(&seed) {
                    new_seed = seed - row[1] + row[0];
                }
            }
            seed = new_seed;
        }
        ans.push(seed);
    }
    ans.iter().min().unwrap().to_string()
}

pub fn process_part2(input: &str) -> String {
    let arr = input.split("\r\n\r\n").collect::<Vec<_>>();
    let old_seeds = arr[0].split(": ").collect::<Vec<_>>()[1]
        .split(" ")
        .map(|seed| seed.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut maps = Vec::new();

    for i in 1..arr.len() {
        maps.push(get_vals(arr[i]));
    }

    let ans = i64::MAX;
    let ans = Mutex::new(ans);

    // let hm: HashSet<i64> = HashSet::new();
    // let hm = Mutex::new(hm);
    let mut x = 0;
    for (i, &val) in old_seeds.iter().enumerate() {
        if i % 2 == 1 {
            x += val;
        }
    }

    let tot = 0;
    let tot = Mutex::new(tot);
    old_seeds.par_iter().enumerate().for_each(|(i, &bag)| {
        if i % 2 == 0 {
            (bag..bag + old_seeds[i + 1])
                .into_par_iter()
                .for_each(|mut seed| {
                    for i in 1..arr.len() {
                        let mut new_seed = seed;
                        let a2 = maps[i - 1].clone();
                        for row in a2 {
                            if (row[1]..(row[1] + row[2])).contains(&seed) {
                                new_seed = seed - row[1] + row[0];
                            }
                        }
                        seed = new_seed;
                    }
                    let mut tot = tot.lock().unwrap();
                    *tot += 1;
                    println!("{} - {} - {}", i, seed, x - *tot);
                    // ans.push(seed);
                    let mut ans = ans.lock().unwrap();
                    if seed < *ans {
                        *ans = seed;
                    }
                })
        }
    });
    let ans = ans.lock().unwrap();
    println!("ans:- {:?}", ans);

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
        assert_eq!(result, "35");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "46");
    }
}
