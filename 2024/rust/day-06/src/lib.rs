use std::collections::HashSet;

pub fn process_part1(input: &str) -> String {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut c: i32 = 0;
    let mut r: i32 = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                r = i as i32;
                c = j as i32;
                break;
            }
        }
    }
    let mut dr = -1;
    let mut dc = 0;

    map[r as usize][c as usize] = 'X';

    loop {
        if r + dr < 0 || r + dr >= map.len() as i32 || c + dc < 0 || c + dc >= map[0].len() as i32 {
            break;
        }

        if map[(r + dr) as usize][(c + dc) as usize] == '#' {
            (dc, dr) = (-dr, dc);
        } else {
            r += dr;
            c += dc;
        }

        map[r as usize][c as usize] = 'X';
    }

    let mut res = 0;

    for row in map.iter() {
        for c in row.iter() {
            if *c == 'X' {
                res += 1;
            }
        }
    }

    res.to_string()
}

fn check_loop(map: Vec<Vec<char>>, mut r: i32, mut c: i32) -> bool {
    let mut dr = -1;
    let mut dc = 0;

    let mut seen: HashSet<(i32, i32, i32, i32)> = HashSet::new();
    loop {
        seen.insert((r, c, dr, dc));
        if r + dr < 0 || r + dr >= map.len() as i32 || c + dc < 0 || c + dc >= map[0].len() as i32 {
            return false;
        }

        if map[(r + dr) as usize][(c + dc) as usize] == '#' {
            (dc, dr) = (-dr, dc);
        } else {
            r += dr;
            c += dc;
        }

        if seen.contains(&(r, c, dr, dc)) {
            return true;
        }
    }
}

pub fn process_part2(input: &str) -> String {
    let mut map = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut c: i32 = 0;
    let mut r: i32 = 0;

    for (i, row) in map.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                r = i as i32;
                c = j as i32;
                break;
            }
        }
    }

    let mut count = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if map[i][j] == '.' {
                map[i][j] = '#';
                if check_loop(map.clone(), r, c) {
                    count += 1;
                }
                map[i][j] = '.';
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
        assert_eq!(result, "41");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "6");
    }
}
