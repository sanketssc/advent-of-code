#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
// Recursive Approach
// fn mark_outside(vis: &mut Vec<Vec<i32>>, x: usize, y: usize) {
//     if vis[x][y] == 2 || vis[x][y] == 1 {
//         return;
//     }
//     vis[x][y] = 2;
//     let x_len = vis.len();
//     let y_len = vis[0].len();

//     if x + 1 < x_len {
//         mark_outside(vis, x + 1, y);
//     }
//     if x != 0 {
//         mark_outside(vis, x - 1, y);
//     }
//     if y + 1 < y_len {
//         mark_outside(vis, x, y + 1);
//     }
//     if y != 0 {
//         mark_outside(vis, x, y - 1);
//     }

//     return;
// }

// Iterative Approach
fn mark_outside(vis: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    let mut stack = vec![(x, y)];

    while let Some((x, y)) = stack.pop() {
        if vis[x][y] == 2 || vis[x][y] == 1 {
            continue;
        }
        vis[x][y] = 2;
        let x_len = vis.len();
        let y_len = vis[0].len();

        if x + 1 < x_len {
            stack.push((x + 1, y));
        }
        if x != 0 {
            stack.push((x - 1, y));
        }
        if y + 1 < y_len {
            stack.push((x, y + 1));
        }
        if y != 0 {
            stack.push((x, y - 1));
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let mut arr = input
        .lines()
        .map(|line| line.split("").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    arr.iter_mut().for_each(|row| {
        row.remove(row.len() - 1);
        row.remove(0);
    });
    let mut start = Point { x: 0, y: 0 };
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if arr[i][j] == "S" {
                start.x = i as i32;
                start.y = j as i32;
            }
        }
    }
    let mut distance = 1;

    let mut s1 = start.clone();
    let mut s2 = start.clone();

    let mut dir1 = Direction::Up;
    let mut dir2 = Direction::Right;
    let mut s1_set = false;
    let mut s2_set = false;
    if arr[start.x as usize + 1][start.y as usize] == "L"
        || arr[start.x as usize + 1][start.y as usize] == "J"
        || arr[start.x as usize + 1][start.y as usize] == "|"
    {
        s1.x = s1.x + 1;
        dir1 = Direction::Down;
        s1_set = true;
    }
    if arr[start.x as usize][start.y as usize + 1] == "7"
        || arr[start.x as usize][start.y as usize + 1] == "J"
        || arr[start.x as usize][start.y as usize + 1] == "-"
    {
        if s1_set {
            s2.y = s2.y + 1;
            dir2 = Direction::Right;
            s2_set = true
        } else {
            s1.y = s1.y + 1;
            dir1 = Direction::Right;
            s1_set = true;
        }
    }
    if !s2_set {
        if arr[start.x as usize - 1][start.y as usize] == "7"
            || arr[start.x as usize - 1][start.y as usize] == "F"
            || arr[start.x as usize - 1][start.y as usize] == "|"
        {
            if s1_set {
                s2.x = s2.x - 1;
                dir2 = Direction::Up;
                s2_set = true
            } else {
                s1.x = s1.x - 1;
                dir1 = Direction::Up;
                s1_set = true;
            }
        }
    }
    if !s2_set {
        if arr[start.x as usize][start.y as usize - 1] == "L"
            || arr[start.x as usize][start.y as usize - 1] == "F"
            || arr[start.x as usize][start.y as usize - 1] == "-"
        {
            if s1_set {
                s2.y = s2.y - 1;
                dir2 = Direction::Left;
            } else {
                s1.y = s1.y - 1;
                dir1 = Direction::Left;
            }
        }
    }

    loop {
        let s1_old = s1.clone();
        let s2_old = s2.clone();
        match dir1 {
            Direction::Down => {
                if arr[s1.x as usize][s1.y as usize] == "L" {
                    dir1 = Direction::Right;
                    s1.y = s1.y + 1;
                } else if arr[s1.x as usize][s1.y as usize] == "J" {
                    dir1 = Direction::Left;
                    s1.y = s1.y - 1;
                } else {
                    s1.x = s1.x + 1;
                }
            }
            Direction::Left => {
                if arr[s1.x as usize][s1.y as usize] == "L" {
                    dir1 = Direction::Up;
                    s1.x = s1.x - 1;
                } else if arr[s1.x as usize][s1.y as usize] == "F" {
                    dir1 = Direction::Down;
                    s1.x = s1.x + 1;
                } else {
                    s1.y = s1.y - 1;
                }
            }
            Direction::Right => {
                if arr[s1.x as usize][s1.y as usize] == "J" {
                    dir1 = Direction::Up;
                    s1.x = s1.x - 1;
                } else if arr[s1.x as usize][s1.y as usize] == "7" {
                    dir1 = Direction::Down;
                    s1.x = s1.x + 1;
                } else {
                    s1.y = s1.y + 1;
                }
            }
            Direction::Up => {
                if arr[s1.x as usize][s1.y as usize] == "F" {
                    dir1 = Direction::Right;
                    s1.y = s1.y + 1;
                } else if arr[s1.x as usize][s1.y as usize] == "7" {
                    dir1 = Direction::Left;
                    s1.y = s1.y - 1;
                } else {
                    s1.x = s1.x - 1;
                }
            }
        };
        match dir2 {
            Direction::Down => {
                if arr[s2.x as usize][s2.y as usize] == "L" {
                    dir2 = Direction::Right;
                    s2.y = s2.y + 1;
                } else if arr[s2.x as usize][s2.y as usize] == "J" {
                    dir2 = Direction::Left;
                    s2.y = s2.y - 1;
                } else {
                    s2.x = s2.x + 1;
                }
            }
            Direction::Left => {
                if arr[s2.x as usize][s2.y as usize] == "L" {
                    dir2 = Direction::Up;
                    s2.x = s2.x - 1;
                } else if arr[s2.x as usize][s2.y as usize] == "F" {
                    dir2 = Direction::Down;
                    s2.x = s2.x + 1;
                } else {
                    s2.y = s2.y - 1;
                }
            }
            Direction::Right => {
                if arr[s2.x as usize][s2.y as usize] == "J" {
                    dir2 = Direction::Up;
                    s2.x = s2.x - 1;
                } else if arr[s2.x as usize][s2.y as usize] == "7" {
                    dir2 = Direction::Down;
                    s2.x = s2.x + 1;
                } else {
                    s2.y = s2.y + 1;
                }
            }
            Direction::Up => {
                if arr[s2.x as usize][s2.y as usize] == "F" {
                    dir2 = Direction::Right;
                    s2.y = s2.y + 1;
                } else if arr[s2.x as usize][s2.y as usize] == "7" {
                    dir2 = Direction::Left;
                    s2.y = s2.y - 1;
                } else {
                    s2.x = s2.x - 1;
                }
            }
        }
        distance += 1;
        if s1 == s2 || s1_old == s2 || s2_old == s1 {
            break;
        }
    }
    distance.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut arr = input
        .lines()
        .map(|line| line.split("").collect::<Vec<_>>())
        .collect::<Vec<_>>();
    arr.iter_mut().for_each(|row| {
        row.remove(row.len() - 1);
        row.remove(0);
    });
    let mut start = Point { x: 0, y: 0 };
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if arr[i][j] == "S" {
                start.x = i as i32;
                start.y = j as i32;
            }
        }
    }

    let mut s1 = start.clone();
    let mut s2 = start.clone();

    let mut dir1 = Direction::Up;
    let mut dir2 = Direction::Right;
    let mut s1_set = false;
    if arr[start.x as usize + 1][start.y as usize] == "L"
        || arr[start.x as usize + 1][start.y as usize] == "J"
        || arr[start.x as usize + 1][start.y as usize] == "|"
    {
        s1.x = s1.x + 1;
        dir1 = Direction::Down;
        s1_set = true;
    }
    if arr[start.x as usize][start.y as usize + 1] == "7"
        || arr[start.x as usize][start.y as usize + 1] == "J"
        || arr[start.x as usize][start.y as usize + 1] == "-"
    {
        if s1_set {
            s2.y = s2.y + 1;
            dir2 = Direction::Right;
        } else {
            s1.y = s1.y + 1;
            dir1 = Direction::Right;
            s1_set = true;
        }
    }
    if start.x != 0 {
        if arr[start.x as usize - 1][start.y as usize] == "7"
            || arr[start.x as usize - 1][start.y as usize] == "F"
            || arr[start.x as usize - 1][start.y as usize] == "|"
        {
            if s1_set {
                s2.x = s2.x - 1;
                dir2 = Direction::Up;
            } else {
                s1.x = s1.x - 1;
                dir1 = Direction::Up;
                s1_set = true;
            }
        }
    }
    if start.y != 0 {
        if arr[start.x as usize][start.y as usize - 1] == "L"
            || arr[start.x as usize][start.y as usize - 1] == "F"
            || arr[start.x as usize][start.y as usize - 1] == "-"
        {
            if s1_set {
                s2.y = s2.y - 1;
                dir2 = Direction::Left;
            } else {
                s1.y = s1.y - 1;
                dir1 = Direction::Left;
            }
        }
    }

    let mut vis: Vec<Vec<i32>> = vec![vec![0; arr[0].len() * 2]; arr.len() * 2];
    vis[s1.x as usize * 2][s1.y as usize * 2] = 1;
    vis[s2.x as usize * 2][s2.y as usize * 2] = 1;
    vis[start.x as usize * 2][start.y as usize * 2] = 1;

    let mut s1_mid: Point = s1.clone();
    if s1.x > start.x {
        s1_mid.x = start.x * 2 + (s1.x * 2 - start.x * 2) / 2;
    } else {
        s1_mid.x = s1.x * 2 + (start.x * 2 - s1.x * 2) / 2;
    }
    if s1.y > start.y {
        s1_mid.y = start.y * 2 + (s1.y * 2 - start.y * 2) / 2;
    } else {
        s1_mid.y = s1.y * 2 + (start.y * 2 - s1.y * 2) / 2;
    }
    let mut s2_mid: Point = s2.clone();
    if s2.x > start.x {
        s2_mid.x = start.x * 2 + (s2.x * 2 - start.x * 2) / 2;
    } else {
        s2_mid.x = s2.x * 2 + (start.x * 2 - s2.x * 2) / 2;
    }
    if s2.y > start.y {
        s2_mid.y = start.y * 2 + (s2.y * 2 - start.y * 2) / 2;
    } else {
        s2_mid.y = s2.y * 2 + (start.y * 2 - s2.y * 2) / 2;
    }
    vis[s1_mid.x as usize][s1_mid.y as usize] = 1;
    vis[s2_mid.x as usize][s2_mid.y as usize] = 1;

    loop {
        let s1_old = s1.clone();
        let s2_old = s2.clone();
        match dir1 {
            Direction::Down => {
                if arr[s1.x as usize][s1.y as usize] == "L" {
                    dir1 = Direction::Right;
                    s1.y = s1.y + 1;
                } else if arr[s1.x as usize][s1.y as usize] == "J" {
                    dir1 = Direction::Left;
                    s1.y = s1.y - 1;
                } else {
                    s1.x = s1.x + 1;
                }
            }
            Direction::Left => {
                if arr[s1.x as usize][s1.y as usize] == "L" {
                    dir1 = Direction::Up;
                    s1.x = s1.x - 1;
                } else if arr[s1.x as usize][s1.y as usize] == "F" {
                    dir1 = Direction::Down;
                    s1.x = s1.x + 1;
                } else {
                    s1.y = s1.y - 1;
                }
            }
            Direction::Right => {
                if arr[s1.x as usize][s1.y as usize] == "J" {
                    dir1 = Direction::Up;
                    s1.x = s1.x - 1;
                } else if arr[s1.x as usize][s1.y as usize] == "7" {
                    dir1 = Direction::Down;
                    s1.x = s1.x + 1;
                } else {
                    s1.y = s1.y + 1;
                }
            }
            Direction::Up => {
                if arr[s1.x as usize][s1.y as usize] == "F" {
                    dir1 = Direction::Right;
                    s1.y = s1.y + 1;
                } else if arr[s1.x as usize][s1.y as usize] == "7" {
                    dir1 = Direction::Left;
                    s1.y = s1.y - 1;
                } else {
                    s1.x = s1.x - 1;
                }
            }
        };
        match dir2 {
            Direction::Down => {
                if arr[s2.x as usize][s2.y as usize] == "L" {
                    dir2 = Direction::Right;
                    s2.y = s2.y + 1;
                } else if arr[s2.x as usize][s2.y as usize] == "J" {
                    dir2 = Direction::Left;
                    s2.y = s2.y - 1;
                } else {
                    s2.x = s2.x + 1;
                }
            }
            Direction::Left => {
                if arr[s2.x as usize][s2.y as usize] == "L" {
                    dir2 = Direction::Up;
                    s2.x = s2.x - 1;
                } else if arr[s2.x as usize][s2.y as usize] == "F" {
                    dir2 = Direction::Down;
                    s2.x = s2.x + 1;
                } else {
                    s2.y = s2.y - 1;
                }
            }
            Direction::Right => {
                if arr[s2.x as usize][s2.y as usize] == "J" {
                    dir2 = Direction::Up;
                    s2.x = s2.x - 1;
                } else if arr[s2.x as usize][s2.y as usize] == "7" {
                    dir2 = Direction::Down;
                    s2.x = s2.x + 1;
                } else {
                    s2.y = s2.y + 1;
                }
            }
            Direction::Up => {
                if arr[s2.x as usize][s2.y as usize] == "F" {
                    dir2 = Direction::Right;
                    s2.y = s2.y + 1;
                } else if arr[s2.x as usize][s2.y as usize] == "7" {
                    dir2 = Direction::Left;
                    s2.y = s2.y - 1;
                } else {
                    s2.x = s2.x - 1;
                }
            }
        }
        let mut s1_mid: Point = s1.clone();
        if s1.x > s1_old.x {
            s1_mid.x = s1_old.x * 2 + (s1.x * 2 - s1_old.x * 2) / 2;
        } else {
            s1_mid.x = s1.x * 2 + (s1_old.x * 2 - s1.x * 2) / 2;
        }
        if s1.y > s1_old.y {
            s1_mid.y = s1_old.y * 2 + (s1.y * 2 - s1_old.y * 2) / 2;
        } else {
            s1_mid.y = s1.y * 2 + (s1_old.y * 2 - s1.y * 2) / 2;
        }
        let mut s2_mid: Point = s2.clone();
        if s2.x > s2_old.x {
            s2_mid.x = s2_old.x * 2 + (s2.x * 2 - s2_old.x * 2) / 2;
        } else {
            s2_mid.x = s2.x * 2 + (s2_old.x * 2 - s2.x * 2) / 2;
        }
        if s2.y > s2_old.y {
            s2_mid.y = s2_old.y * 2 + (s2.y * 2 - s2_old.y * 2) / 2;
        } else {
            s2_mid.y = s2.y * 2 + (s2_old.y * 2 - s2.y * 2) / 2;
        }
        vis[s1_mid.x as usize][s1_mid.y as usize] = 1;
        vis[s2_mid.x as usize][s2_mid.y as usize] = 1;

        vis[s1.x as usize * 2][s1.y as usize * 2] = 1;
        vis[s2.x as usize * 2][s2.y as usize * 2] = 1;
        if s1 == s2 || s1_old == s2 || s2_old == s1 {
            break;
        }
    }
    let vis_len = vis.len();
    let vis_row_len = vis[0].len();
    for i in 0..vis.len() {
        if vis[i][0] == 0 {
            mark_outside(&mut vis, i, 0);
        }
        if vis[i][vis[0].len() - 1] == 0 {
            mark_outside(&mut vis, i, vis_row_len - 1);
        }
    }

    for i in 0..vis[0].len() {
        if vis[0][i] == 0 {
            mark_outside(&mut vis, 0, i);
        }
        if vis[vis_len - 1][i] == 0 {
            mark_outside(&mut vis, vis_len - 1, i);
        }
    }

    let mut count = 0;
    for i in 0..vis_len {
        for j in 0..vis_row_len {
            if vis[i][j] == 0 {
                if i % 2 == 0 && j % 2 == 0 {
                    count += 1;
                }
            }
        }
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "8");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "8");
    }
}
