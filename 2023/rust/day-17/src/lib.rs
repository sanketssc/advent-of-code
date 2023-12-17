use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

type Cache = HashMap<(usize, usize, usize), i32>;

pub fn process_part1(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| {
            let mut arr = line.split("").collect::<Vec<&str>>();
            arr.remove(0);
            arr.remove(arr.len() - 1);
            arr.iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<_>>>();

    let l = vec![0, -1];
    let q = vec![0, 1];
    let u = vec![-1, 0];
    let d = vec![1, 0];

    let dir = vec![q, d, l, u];

    let start = (0, 0, 0);
    let start2 = (0, 0, 1);
    let end = (arr.len() - 1, arr[0].len() - 1);
    let mut h = BinaryHeap::new();
    h.push((Reverse(0), start));
    h.push((Reverse(0), start2));

    let mut dist = HashMap::new();
    dist.insert(start, 0);
    dist.insert(start2, 0);

    let mut cache: Cache = HashMap::new();

    while let Some((Reverse(cost), cur)) = h.pop() {
        if let Some(d) = dist.get(&cur).cloned() {
            if d != cost {
                continue;
            }
        }

        if (cur.0, cur.1) == end {
            return cost.to_string();
        }

        let opts = if cur.2 == 0 || cur.2 == 2 {
            vec![1, 3]
        } else {
            vec![0, 2]
        };

        for &idx in &opts {
            let mut nc = cost;
            for l in 1..4 {
                let nx = (cur.0 as i32 + l as i32 * dir[idx][0]) as usize;
                let ny = (cur.1 as i32 + l as i32 * dir[idx][1]) as usize;

                if nx < arr.len() && ny < arr[0].len() {
                    nc += arr[nx][ny];

                    if let Some(&cached_dist) = cache.get(&(nx, ny, idx)) {
                        if nc >= cached_dist {
                            continue;
                        }
                    }

                    if dist
                        .get(&(nx, ny, idx))
                        .cloned()
                        .unwrap_or(i32::max_value())
                        > nc
                    {
                        dist.insert((nx, ny, idx), nc);
                        h.push((Reverse(nc), (nx, ny, idx)));

                        cache.insert((nx, ny, idx), nc);
                    }
                }
            }
        }
    }

    u128::MAX.to_string()
}

pub fn process_part2(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| {
            let mut arr = line.split("").collect::<Vec<&str>>();
            arr.remove(0);
            arr.remove(arr.len() - 1);
            arr.iter()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<_>>>();

    let l = vec![0, -1];
    let q = vec![0, 1];
    let u = vec![-1, 0];
    let d = vec![1, 0];

    let dir = vec![q, d, l, u];

    let start = (0, 0, 0);
    let start2 = (0, 0, 1);
    let end = (arr.len() - 1, arr[0].len() - 1);
    let mut h = BinaryHeap::new();
    h.push((Reverse(0), start));
    h.push((Reverse(0), start2));

    let mut dist = HashMap::new();
    dist.insert(start, 0);
    dist.insert(start2, 0);

    let mut cache: Cache = HashMap::new();

    while let Some((Reverse(cost), cur)) = h.pop() {
        if let Some(d) = dist.get(&cur).cloned() {
            if d != cost {
                continue;
            }
        }

        if (cur.0, cur.1) == end {
            return cost.to_string();
        }

        let opts = if cur.2 == 0 || cur.2 == 2 {
            vec![1, 3]
        } else {
            vec![0, 2]
        };

        for &idx in &opts {
            let mut nc = cost;
            for l in 1..11 {
                let nx = (cur.0 as i32 + l as i32 * dir[idx][0]) as usize;
                let ny = (cur.1 as i32 + l as i32 * dir[idx][1]) as usize;

                if nx < arr.len() && ny < arr[0].len() {
                    nc += arr[nx][ny];

                    if let Some(&cached_dist) = cache.get(&(nx, ny, idx)) {
                        if nc >= cached_dist {
                            continue;
                        }
                    }

                    if l >= 4
                        && dist
                            .get(&(nx, ny, idx))
                            .cloned()
                            .unwrap_or(i32::max_value())
                            > nc
                    {
                        dist.insert((nx, ny, idx), nc);
                        h.push((Reverse(nc), (nx, ny, idx)));

                        cache.insert((nx, ny, idx), nc);
                    }
                }
            }
        }
    }

    u128::MAX.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "102");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "94");
    }
}
