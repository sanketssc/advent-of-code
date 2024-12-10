fn bfs(vec: Vec<Vec<u32>>, current: (usize, usize), done: &mut Vec<(usize, usize)>) {
    if vec[current.1][current.0] == 9 {
        if !done.contains(&current) {
            done.push(current);
        }
        return;
    }

    for [i, j] in [[0, 1], [0, -1], [1, 0], [-1, 0]].iter() {
        let x = current.0 as i32 + i;
        let y = current.1 as i32 + j;
        if x >= 0
            && x < vec[0].len() as i32
            && y >= 0
            && y < vec.len() as i32
            && vec[y as usize][x as usize] == vec[current.1][current.0] + 1
        {
            bfs(vec.clone(), (x as usize, y as usize), done);
        }
    }
}
fn bfs1(vec: Vec<Vec<u32>>, current: (usize, usize), mut count: i32) -> i32 {
    if vec[current.1][current.0] == 9 {
        return count + 1;
    }

    for [i, j] in [[0, 1], [0, -1], [1, 0], [-1, 0]].iter() {
        let x = current.0 as i32 + i;
        let y = current.1 as i32 + j;
        if x >= 0
            && x < vec[0].len() as i32
            && y >= 0
            && y < vec.len() as i32
            && vec[y as usize][x as usize] == vec[current.1][current.0] + 1
        {
            count = bfs1(vec.clone(), (x as usize, y as usize), count);
        }
    }

    count
}

pub fn process_part1(input: &str) -> String {
    let vec = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(100))
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut ans = 0;
    for (j, row) in vec.iter().enumerate() {
        for (i, val) in row.iter().enumerate() {
            if *val == 0 {
                let mut tmp: Vec<(usize, usize)> = vec![];
                bfs(vec.clone(), (i, j), &mut tmp);
                ans += tmp.len() as i32;
            }
        }
    }
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let vec = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    let mut ans = 0;
    for (j, row) in vec.iter().enumerate() {
        for (i, val) in row.iter().enumerate() {
            if *val == 0 {
                ans += bfs1(vec.clone(), (i, j), 0);
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
        assert_eq!(result, "36");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "81");
    }
}
