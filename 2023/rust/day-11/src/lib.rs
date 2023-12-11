pub fn process_part1(input: &str) -> String {
    let mut arr = input
        .lines()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    arr.iter_mut().for_each(|row| {
        row.remove(0);
        row.pop();
    });
    let mut empty_rows: Vec<usize> = Vec::new();
    for (i, _) in arr.iter().enumerate() {
        let mut count_hashes = 0;
        for (j, _) in arr[i].iter().enumerate() {
            if arr[i][j] == "#" {
                count_hashes += 1;
            }
        }
        if count_hashes == 0 {
            empty_rows.push(i);
        }
    }
    let mut empty_cols: Vec<usize> = Vec::new();
    for (i, _) in arr[0].iter().enumerate() {
        let mut count_hashes = 0;
        for (j, _) in arr.iter().enumerate() {
            if arr[j][i] == "#" {
                count_hashes += 1;
            }
        }
        if count_hashes == 0 {
            empty_cols.push(i);
        }
    }
    let mut count = 0;
    for i in empty_rows.iter() {
        arr.insert(*i + count, vec!["."; arr[0].len()]);
        count += 1;
    }
    let mut loc_count = 0;
    for i in empty_cols.iter() {
        for j in arr.iter_mut() {
            j.insert(*i + loc_count, ".");
        }
        loc_count += 1;
    }
    let mut hash_vec: Vec<(usize, usize)> = Vec::new();
    for (i, _) in arr.iter().enumerate() {
        for (j, _) in arr[i].iter().enumerate() {
            if arr[i][j] == "#" {
                hash_vec.push((i, j));
            }
        }
    }
    let mut ans = 0;

    for i in 0..hash_vec.len() {
        for j in i + 1..hash_vec.len() {
            if hash_vec[i].0 > hash_vec[j].0 {
                ans = ans + hash_vec[i].0 - hash_vec[j].0;
            } else {
                ans = ans + hash_vec[j].0 - hash_vec[i].0;
            }
            if hash_vec[i].1 > hash_vec[j].1 {
                ans = ans + hash_vec[i].1 - hash_vec[j].1;
            } else {
                ans = ans + hash_vec[j].1 - hash_vec[i].1;
            }
        }
    }
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut arr = input
        .lines()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    arr.iter_mut().for_each(|row| {
        row.remove(0);
        row.pop();
    });
    let mut empty_rows: Vec<usize> = Vec::new();
    for (i, _) in arr.iter().enumerate() {
        let mut count_hashes = 0;
        for (j, _) in arr[i].iter().enumerate() {
            if arr[i][j] == "#" {
                count_hashes += 1;
            }
        }
        if count_hashes == 0 {
            empty_rows.push(i);
        }
    }
    let mut empty_cols: Vec<usize> = Vec::new();
    for (i, _) in arr[0].iter().enumerate() {
        let mut count_hashes = 0;
        for (j, _) in arr.iter().enumerate() {
            if arr[j][i] == "#" {
                count_hashes += 1;
            }
        }
        if count_hashes == 0 {
            empty_cols.push(i);
        }
    }
    let mut hash_vec: Vec<(usize, usize)> = Vec::new();
    for (i, _) in arr.iter().enumerate() {
        for (j, _) in arr[i].iter().enumerate() {
            if arr[i][j] == "#" {
                hash_vec.push((i, j));
            }
        }
    }
    let mut ans = 0;

    for i in 0..hash_vec.len() {
        for j in i + 1..hash_vec.len() {
            if hash_vec[i].0 > hash_vec[j].0 {
                ans = ans + hash_vec[i].0 - hash_vec[j].0;
                for &it in empty_rows.iter() {
                    if it > hash_vec[j].0 && it < hash_vec[i].0 {
                        ans = ans + 999999;
                    }
                }
            } else {
                ans = ans + hash_vec[j].0 - hash_vec[i].0;
                for &it in empty_rows.iter() {
                    if it < hash_vec[j].0 && it > hash_vec[i].0 {
                        ans = ans + 999999;
                    }
                }
            }
            if hash_vec[i].1 > hash_vec[j].1 {
                ans = ans + hash_vec[i].1 - hash_vec[j].1;
                for &it in empty_cols.iter() {
                    if it > hash_vec[j].1 && it < hash_vec[i].1 {
                        ans = ans + 999999;
                    }
                }
            } else {
                ans = ans + hash_vec[j].1 - hash_vec[i].1;
                for &it in empty_cols.iter() {
                    if it < hash_vec[j].1 && it > hash_vec[i].1 {
                        ans = ans + 999999;
                    }
                }
            }
        }
    }
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
        assert_eq!(result, "374");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "82000210");
    }
}
