fn get_diffs_right(row: Vec<i32>) -> i32 {
    let mut count = 0;
    for &i in row.iter() {
        if i == 0 {
            count += 1;
        }
    }
    if count == row.len() {
        0
    } else {
        let mut new_vec: Vec<i32> = Vec::new();
        for i in 1..row.len() {
            new_vec.push(row[i] - row[i - 1]);
        }
        let val = get_diffs_right(new_vec);
        val + row[row.len() - 1]
    }
}

fn get_diffs_left(row: Vec<i32>) -> i32 {
    let mut count = 0;
    for &i in row.iter() {
        if i == 0 {
            count += 1;
        }
    }
    if count == row.len() {
        0
    } else {
        let mut new_vec: Vec<i32> = Vec::new();
        for i in 1..row.len() {
            new_vec.push(row[i] - row[i - 1]);
        }
        let val = get_diffs_left(new_vec);
        row[0] - val
    }
}

pub fn process_part1(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    arr.iter().for_each(|row| {
        let x = get_diffs_right(row.to_vec());
        ans += x;
    });
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut ans = 0;
    arr.iter().for_each(|row| {
        let x = get_diffs_left(row.to_vec());
        ans += x;
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
        assert_eq!(result, "114");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "2");
    }
}
