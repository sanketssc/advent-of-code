use std::collections::{HashSet, VecDeque};

pub fn process_part1(input: &str) -> String {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut points = vec![];
    for line in input.lines() {
        let content = line.split_once(" ").unwrap();
        let parse_pair = |s: &str| {
            let mut parts = s.split('=').nth(1).unwrap().split(',');
            (
                parts.next().unwrap().parse::<i128>().unwrap(),
                parts.next().unwrap().parse::<i128>().unwrap(),
            )
        };

        let point = parse_pair(content.0);
        let velocity = parse_pair(content.1);

        if point.0 > max_x {
            max_x = point.0;
        }
        if point.1 > max_y {
            max_y = point.1;
        }

        points.push((point, velocity));
    }
    max_x += 1;
    max_y += 1;

    let mut vec = vec![vec![0; max_x as usize]; max_y as usize];
    for point in points {
        vec[((((point.0 .1 + point.1 .1 * 100) % max_y) + max_y) % max_y) as usize]
            [((((point.0 .0 + point.1 .0 * 100) % max_x) + max_x) % max_x) as usize] += 1;
    }
    let mut sum1 = 0;
    let mut sum2 = 0;
    let mut sum3 = 0;
    let mut sum4 = 0;

    for i in 0..max_y / 2 {
        for j in 0..max_x / 2 {
            sum1 += vec[i as usize][j as usize];
        }
    }

    for i in 0..max_y / 2 {
        for j in max_x / 2 + 1..max_x {
            sum2 += vec[i as usize][j as usize];
        }
    }

    for i in max_y / 2 + 1..max_y {
        for j in 0..max_x / 2 {
            sum3 += vec[i as usize][j as usize];
        }
    }

    for i in max_y / 2 + 1..max_y {
        for j in max_x / 2 + 1..max_x {
            sum4 += vec[i as usize][j as usize];
        }
    }

    (sum1 * sum2 * sum3 * sum4).to_string()
}

fn bfs(start: (i32, i32), is_valid: impl Fn((i32, i32)) -> bool) -> HashSet<(i32, i32)> {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        let (x, y) = current;
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neighbor = (x + dx, y + dy);
            if is_valid(neighbor) && !visited.contains(&neighbor) {
                queue.push_back(neighbor);
                visited.insert(neighbor);
            }
        }
    }

    visited
}

pub fn process_part2(input: &str) -> String {
    let mut max_x = 0;
    let mut max_y = 0;
    let mut points = vec![];
    for line in input.lines() {
        let content = line.split_once(" ").unwrap();
        let parse_pair = |s: &str| {
            let mut parts = s.split('=').nth(1).unwrap().split(',');
            (
                parts.next().unwrap().parse::<i128>().unwrap(),
                parts.next().unwrap().parse::<i128>().unwrap(),
            )
        };

        let point = parse_pair(content.0);
        let velocity = parse_pair(content.1);

        if point.0 > max_x {
            max_x = point.0;
        }
        if point.1 > max_y {
            max_y = point.1;
        }

        points.push((point, velocity));
    }
    max_x += 1;
    max_y += 1;

    let center = (max_x / 2, max_y / 2);

    for i in 0.. {
        let mut vec = vec![vec![0; max_x as usize]; max_y as usize];
        for point in &points {
            vec[((((point.0 .1 + point.1 .1 * i) % max_y) + max_y) % max_y) as usize]
                [((((point.0 .0 + point.1 .0 * i) % max_x) + max_x) % max_x) as usize] += 1;
        }

        if bfs((center.0 as i32, center.1 as i32), |(x, y)| {
            0 <= x
                && x < max_x as i32
                && 0 <= y
                && y < max_y as i32
                && vec[y as usize][x as usize] > 0
        })
        .len()
            > 30
        {
            return i.to_string();
        }
    }

    (-1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "12");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "8149");
    }
}
