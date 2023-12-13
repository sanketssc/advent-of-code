pub fn process_part1(input: &str) -> String {
    let patterns = input.split("\r\n\r\n").collect::<Vec<_>>();
    // println!("{:?}", patterns);
    let mut horizontal_sum = 0;
    let mut vertical_sum = 0;
    patterns.iter().for_each(|pattern| {
        let horizontal = pattern.lines().collect::<Vec<_>>();
        let mut horizontal_set = false;
        for i in 1..horizontal.len() {
            if i == 1 && horizontal[i] == horizontal[i - 1] {
                horizontal_sum += i;
                horizontal_set = true;
                break;
            } else if horizontal[i] == horizontal[i - 1] {
                let mut top = i - 2;
                let mut bot = i + 1;
                let mut is_symmetric = true;

                while bot < horizontal.len() {
                    if horizontal[top] != horizontal[bot] {
                        is_symmetric = false;
                        break;
                    }
                    if top > 0 {
                        top -= 1;
                    } else {
                        break;
                    }
                    bot += 1;
                }
                if is_symmetric {
                    horizontal_sum += i;
                    horizontal_set = true;
                    break;
                }
            }
        }
        if !horizontal_set {
            let mut vertical: Vec<String> = Vec::new();
            for i in 0..horizontal[0].len() {
                let mut temp = String::new();
                for j in 0..horizontal.len() {
                    temp.push(horizontal[j].chars().nth(i).unwrap());
                }
                vertical.push(temp);
            }
            for i in 1..vertical.len() {
                if i == 1 && vertical[i] == vertical[i - 1] {
                    vertical_sum += i;
                    break;
                } else if vertical[i] == vertical[i - 1] {
                    let mut top = i - 2;
                    let mut bot = i + 1;
                    let mut is_symmetric = true;

                    while bot < vertical.len() {
                        if vertical[top] != vertical[bot] {
                            is_symmetric = false;
                            break;
                        }
                        if top > 0 {
                            top -= 1;
                        } else {
                            break;
                        }
                        bot += 1;
                    }
                    if is_symmetric {
                        vertical_sum += i;
                        break;
                    }
                }
            }
        }
    });
    let ans = horizontal_sum * 100 + vertical_sum;

    ans.to_string()
}

fn find_mirror(grid: &Vec<&str>) -> usize {
    for r in 1..grid.len() {
        let above: Vec<&str> = grid[..r].iter().rev().cloned().collect();
        let below: Vec<&str> = grid[r..].to_vec();

        let diff_count = above
            .iter()
            .zip(below.iter())
            .map(|(a, b)| {
                a.chars()
                    .zip(b.chars())
                    .filter(|&(ca, cb)| ca != cb)
                    .count()
            })
            .sum::<usize>();

        if diff_count == 1 {
            return r;
        }
    }

    0
}

pub fn process_part2(input: &str) -> String {
    let patterns: Vec<&str> = input.split("\r\n\r\n").collect();

    let mut total = 0;

    for pattern in patterns {
        let grid: Vec<&str> = pattern.lines().collect();

        let row = find_mirror(&grid);

        total += row * 100;

        let mut temp_strings: Vec<String> = Vec::new();
        let mut vertical: Vec<&str> = Vec::new();
        for i in 0..grid[0].len() {
            let mut temp = String::new();
            for j in 0..grid.len() {
                temp.push_str(&grid[j].chars().nth(i).unwrap().to_string());
            }
            temp_strings.push(temp);
        }

        for temp in &temp_strings {
            vertical.push(temp);
        }
        let col = find_mirror(&vertical);
        total += col;
    }

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "405");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "400");
    }
}
