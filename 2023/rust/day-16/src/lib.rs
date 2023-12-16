enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn move_along(
    arr: &Vec<Vec<char>>,
    vis: &mut Vec<Vec<u32>>,
    mut x: usize,
    mut y: usize,
    mut dir: Dir,
) {
    loop {
        match dir {
            Dir::Up => {
                if vis[x][y] == 1 {
                    break;
                } else {
                    vis[x][y] = 1;
                }
            }
            Dir::Down => {
                if vis[x][y] == 2 {
                    break;
                } else {
                    vis[x][y] = 2;
                }
            }
            Dir::Left => {
                if vis[x][y] == 3 {
                    break;
                } else {
                    vis[x][y] = 3;
                }
            }
            Dir::Right => {
                if vis[x][y] == 4 {
                    break;
                } else {
                    vis[x][y] = 4;
                }
            }
        }

        match dir {
            Dir::Right => {
                if arr[x][y] == '.' {
                    if y + 1 < arr[0].len() {
                        y += 1;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '/' {
                    if x > 0 {
                        x -= 1;
                        dir = Dir::Up;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '\\' {
                    if x + 1 < arr.len() {
                        x += 1;
                        dir = Dir::Down;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '|' {
                    if x > 0 {
                        move_along(arr, vis, x - 1, y, Dir::Up);
                    }
                    if x + 1 < arr.len() {
                        move_along(arr, vis, x + 1, y, Dir::Down);
                    }
                } else if arr[x][y] == '-' {
                    if y > 0 {
                        move_along(arr, vis, x, y - 1, Dir::Left);
                    }
                    if y + 1 < arr[0].len() {
                        move_along(arr, vis, x, y + 1, Dir::Right);
                    }
                }
            }
            Dir::Left => {
                if arr[x][y] == '.' {
                    if y > 0 {
                        y -= 1;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '/' {
                    if x + 1 < arr.len() {
                        x += 1;
                        dir = Dir::Down;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '\\' {
                    if x > 0 {
                        x -= 1;
                        dir = Dir::Up;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '|' {
                    if x > 0 {
                        move_along(arr, vis, x - 1, y, Dir::Up);
                    }
                    if x + 1 < arr.len() {
                        move_along(arr, vis, x + 1, y, Dir::Down);
                    }
                } else if arr[x][y] == '-' {
                    if y > 0 {
                        move_along(arr, vis, x, y - 1, Dir::Left);
                    }
                    if y + 1 < arr[0].len() {
                        move_along(arr, vis, x, y + 1, Dir::Right);
                    }
                }
            }
            Dir::Up => {
                if arr[x][y] == '.' {
                    if x > 0 {
                        x -= 1;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '/' {
                    if y + 1 < arr[0].len() {
                        y += 1;
                        dir = Dir::Right;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '\\' {
                    if y > 0 {
                        y -= 1;
                        dir = Dir::Left;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '|' {
                    if x > 0 {
                        move_along(arr, vis, x - 1, y, Dir::Up);
                    }
                    if x + 1 < arr.len() {
                        move_along(arr, vis, x + 1, y, Dir::Down);
                    }
                } else if arr[x][y] == '-' {
                    if y > 0 {
                        move_along(arr, vis, x, y - 1, Dir::Left);
                    }
                    if y + 1 < arr[0].len() {
                        move_along(arr, vis, x, y + 1, Dir::Right);
                    }
                }
            }
            Dir::Down => {
                if arr[x][y] == '.' {
                    if x + 1 < arr.len() {
                        x += 1;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '/' {
                    if y > 0 {
                        y -= 1;
                        dir = Dir::Left;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '\\' {
                    if y + 1 < arr[0].len() {
                        y += 1;
                        dir = Dir::Right;
                    } else {
                        break;
                    }
                } else if arr[x][y] == '|' {
                    if x > 0 {
                        move_along(arr, vis, x - 1, y, Dir::Up);
                    }
                    if x + 1 < arr.len() {
                        move_along(arr, vis, x + 1, y, Dir::Down);
                    }
                } else if arr[x][y] == '-' {
                    if y > 0 {
                        move_along(arr, vis, x, y - 1, Dir::Left);
                    }
                    if y + 1 < arr[0].len() {
                        move_along(arr, vis, x, y + 1, Dir::Right);
                    }
                }
            }
        }
    }
}

pub fn process_part1(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut vis = vec![vec![0; arr[0].len()]; arr.len()];

    move_along(&arr, &mut vis, 0, 0, Dir::Right);
    let mut count = 0;

    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if vis[i][j] > 0 {
                count += 1;
            }
        }
    }
    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let arr = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut max_count = 0;

    for i in 0..arr.len() {
        let mut vis = vec![vec![0; arr[0].len()]; arr.len()];

        move_along(&arr, &mut vis, i, 0, Dir::Right);
        let mut count = 0;

        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                if vis[i][j] > 0 {
                    count += 1;
                }
            }
        }
        if count > max_count {
            max_count = count;
        }
    }
    for i in 0..arr.len() {
        let mut vis = vec![vec![0; arr[0].len()]; arr.len()];

        move_along(&arr, &mut vis, i, arr[0].len() - 1, Dir::Left);
        let mut count = 0;

        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                if vis[i][j] > 0 {
                    count += 1;
                }
            }
        }
        if count > max_count {
            max_count = count;
        }
    }

    for i in 0..arr[0].len() {
        let mut vis = vec![vec![0; arr[0].len()]; arr.len()];

        move_along(&arr, &mut vis, 0, i, Dir::Down);
        let mut count = 0;

        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                if vis[i][j] > 0 {
                    count += 1;
                }
            }
        }
        if count > max_count {
            max_count = count;
        }
    }

    for i in 0..arr[0].len() {
        let mut vis = vec![vec![0; arr[0].len()]; arr.len()];

        move_along(&arr, &mut vis, arr.len() - 1, i, Dir::Up);
        let mut count = 0;

        for i in 0..arr.len() {
            for j in 0..arr[0].len() {
                if vis[i][j] > 0 {
                    count += 1;
                }
            }
        }
        if count > max_count {
            max_count = count;
        }
    }

    max_count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "46");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "51");
    }
}
