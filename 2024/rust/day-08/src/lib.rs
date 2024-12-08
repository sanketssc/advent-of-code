use std::collections::{HashMap, HashSet};

pub fn process_part1(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col != &'.' {
                //if not present insert or else increment
                antennas
                    .entry(*col)
                    .or_insert(vec![])
                    .push((i as i32, j as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    let mut ans = 0;
    for (_key, value) in antennas.iter() {
        for i in 0..value.len() {
            for j in i + 1..value.len() {
                let diffy = (value[i].1 - value[j].1).abs();
                let diffx = (value[i].0 - value[j].0).abs();

                if value[i].0 > value[j].0 && value[i].1 > value[j].1 {
                    antinodes.insert((value[i].0 + diffx, value[i].1 + diffy));
                    antinodes.insert((value[j].0 - diffx, value[j].1 - diffy));
                } else if value[i].0 > value[j].0 && value[i].1 < value[j].1 {
                    antinodes.insert((value[i].0 + diffx, value[i].1 - diffy));
                    antinodes.insert((value[j].0 - diffx, value[j].1 + diffy));
                } else if value[i].0 < value[j].0 && value[i].1 > value[j].1 {
                    antinodes.insert((value[i].0 - diffx, value[i].1 + diffy));
                    antinodes.insert((value[j].0 + diffx, value[j].1 - diffy));
                } else {
                    antinodes.insert((value[i].0 - diffx, value[i].1 - diffy));
                    antinodes.insert((value[j].0 + diffx, value[j].1 + diffy));
                }
            }
        }
    }

    // check if antinode points are valid for grid dimensions
    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if antinodes.contains(&(i as i32, j as i32)) {
                ans += 1;
            }
        }
    }
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col != &'.' {
                //if not present insert or else increment
                antennas
                    .entry(*col)
                    .or_insert(vec![])
                    .push((i as i32, j as i32));
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    let mut ans = 0;
    for (_key, value) in antennas.iter() {
        for i in 0..value.len() {
            for j in i + 1..value.len() {
                let diffy = (value[i].1 - value[j].1).abs();
                let diffx = (value[i].0 - value[j].0).abs();

                let (mut ix, mut iy, mut jx, mut jy) =
                    (value[i].0, value[i].1, value[j].0, value[j].1);

                antinodes.insert((ix, iy));
                antinodes.insert((jx, jy));

                if value[i].0 > value[j].0 && value[i].1 > value[j].1 {
                    while ix + diffx < grid.len() as i32 && iy + diffy < grid[0].len() as i32 {
                        antinodes.insert((ix + diffx, iy + diffy));
                        ix += diffx;
                        iy += diffy;
                    }
                    while jx - diffx >= 0 && jy - diffy >= 0 {
                        antinodes.insert((jx - diffx, jy - diffy));
                        jx -= diffx;
                        jy -= diffy;
                    }
                } else if value[i].0 > value[j].0 && value[i].1 < value[j].1 {
                    while ix + diffx < grid.len() as i32 && iy - diffy >= 0 {
                        antinodes.insert((ix + diffx, iy - diffy));
                        ix += diffx;
                        iy -= diffy;
                    }
                    while jx - diffx >= 0 && jy + diffy < grid[0].len() as i32 {
                        antinodes.insert((jx - diffx, jy + diffy));
                        jx -= diffx;
                        jy += diffy;
                    }
                } else if value[i].0 < value[j].0 && value[i].1 > value[j].1 {
                    while ix - diffx >= 0 && iy + diffy < grid[0].len() as i32 {
                        antinodes.insert((ix - diffx, iy + diffy));
                        ix -= diffx;
                        iy += diffy;
                    }
                    while jx + diffx < grid.len() as i32 && jy - diffy >= 0 {
                        antinodes.insert((jx + diffx, jy - diffy));
                        jx += diffx;
                        jy -= diffy;
                    }
                } else {
                    while ix - diffx >= 0 && iy - diffy >= 0 {
                        antinodes.insert((ix - diffx, iy - diffy));
                        ix -= diffx;
                        iy -= diffy;
                    }
                    while jx + diffx < grid.len() as i32 && jy + diffy < grid[0].len() as i32 {
                        antinodes.insert((jx + diffx, jy + diffy));
                        jx += diffx;
                        jy += diffy;
                    }
                }
            }
        }
    }

    for (i, row) in grid.iter().enumerate() {
        for (j, _col) in row.iter().enumerate() {
            if antinodes.contains(&(i as i32, j as i32)) {
                ans += 1;
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
        assert_eq!(result, "14");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "34");
    }
}
