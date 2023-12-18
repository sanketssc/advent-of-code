use std::collections::HashSet;

fn dfs(x: usize, y: usize, grid: &mut Vec<Vec<char>>) {
    let mut stack: Vec<(usize, usize)> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    stack.push((x, y));

    while let Some((x, y)) = stack.pop() {
        if x >= grid.len() || y >= grid[0].len() || visited.contains(&(x, y)) || grid[x][y] == '#' {
            continue;
        }
        visited.insert((x, y));
        grid[x][y] = 'O';

        if x + 1 < grid.len() {
            stack.push((x + 1, y));
        }
        if x > 0 {
            stack.push((x - 1, y));
        }
        if y + 1 < grid[0].len() {
            stack.push((x, y + 1));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let mut arr: Vec<(char, i32, &str)> = Vec::new();
    input.lines().for_each(|line| {
        let c: char = line.split(" ").nth(0).unwrap().chars().nth(0).unwrap();
        let i: i32 = line.split(" ").nth(1).unwrap().parse().unwrap();
        let s: &str = line.split(" ").nth(2).unwrap();
        arr.push((c, i, s));
    });
    let mut start: (i32, i32) = (0, 0);
    let mut points: Vec<(i32, i32)> = Vec::new();
    points.push(start);
    arr.iter().for_each(|(c, i, _s)| match c {
        'R' => {
            for _ in 0..*i {
                points.push((start.0, start.1 + 1));
                start.1 += 1;
            }
        }
        'L' => {
            for _ in 0..*i {
                points.push((start.0, start.1 - 1));
                start.1 -= 1;
            }
        }
        'U' => {
            for _ in 0..*i {
                points.push((start.0 - 1, start.1));
                start.0 -= 1;
            }
        }
        'D' => {
            for _ in 0..*i {
                points.push((start.0 + 1, start.1));
                start.0 += 1;
            }
        }
        _ => panic!("Unknown direction"),
    });

    let left_most_point = points.iter().min_by_key(|(_x, y)| y).unwrap().1;
    let right_most_point = points.iter().max_by_key(|(_x, y)| y).unwrap().1;
    let top_most_point = points.iter().min_by_key(|(x, _y)| x).unwrap().0;
    let bottom_most_point = points.iter().max_by_key(|(x, _y)| x).unwrap().0;

    let mut grid: Vec<Vec<char>> = vec![
        vec!['.'; (right_most_point - left_most_point + 1) as usize];
        (bottom_most_point - top_most_point + 1) as usize
    ];
    if top_most_point < 0 {
        start.0 += top_most_point.abs();
    }
    if left_most_point < 0 {
        start.1 += left_most_point.abs();
    }
    for (x, y) in points {
        grid[(x + start.0) as usize][(y + start.1) as usize] = '#';
    }

    for i in 0..grid.len() {
        if grid[i][0] == '.' {
            dfs(i, 0, &mut grid);
        }
        if grid[i][grid[0].len() - 1] == '.' {
            dfs(i, grid[0].len() - 1, &mut grid);
        }
    }

    let mut count = 0;
    for i in grid {
        for j in i {
            if j == '#' || j == '.' {
                count += 1;
            }
        }
    }
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut points: Vec<(i128, i128)> = vec![(0, 0)];
    let dirs: Vec<(i128, i128)> = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];

    let mut b: i128 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let x = parts[2][2..parts[2].len() - 1].to_string();
        let (dr, dc) = dirs[x.chars().last().unwrap() as usize - '0' as usize];

        let n = i128::from_str_radix(&x[..x.len() - 1], 16).unwrap();
        b += n;
        let (r, c) = points.last().unwrap();
        points.push((r + dr * n, c + dc * n));
    }

    let a = points
        .iter()
        .enumerate()
        .map(|(i, &p)| {
            let prev: (i128, i128);
            if i == 0 {
                prev = points[points.len() - 1];
            } else {
                prev = points[i - 1];
            }
            let next = points[(i + 1) % points.len()];
            p.0 * (prev.1 - next.1)
        })
        .sum::<i128>()
        .abs()
        / 2;

    let i = a - (b / 2) + 1;
    let ans = i + b;

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
        assert_eq!(result, "62");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "952408144115");
    }
}
