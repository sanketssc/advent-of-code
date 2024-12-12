use std::collections::HashMap;

fn find(i: usize, j: usize, vec: &Vec<Vec<char>>, visited: &mut Vec<Vec<char>>) -> i32 {
    if visited[i][j] != ' ' {
        return 0;
    }
    visited[i][j] = vec[i][j];
    let mut perimeter;
    let mut surrounded_by = 0;
    if i > 0 {
        if vec[i - 1][j] == visited[i][j] {
            surrounded_by += 1;
        }
    }
    if i < vec.len() - 1 {
        if vec[i + 1][j] == visited[i][j] {
            surrounded_by += 1;
        }
    }
    if j > 0 {
        if vec[i][j - 1] == visited[i][j] {
            surrounded_by += 1;
        }
    }
    if j < vec[0].len() - 1 {
        if vec[i][j + 1] == visited[i][j] {
            surrounded_by += 1;
        }
    }

    if surrounded_by == 0 {
        perimeter = 4;
    } else if surrounded_by == 1 {
        perimeter = 3;
    } else if surrounded_by == 2 {
        perimeter = 2;
    } else if surrounded_by == 3 {
        perimeter = 1;
    } else {
        perimeter = 0;
    }

    if i > 0 {
        if vec[i - 1][j] == vec[i][j] {
            perimeter += find(i - 1, j, vec, visited);
        }
    }
    if i < vec.len() - 1 {
        if vec[i + 1][j] == vec[i][j] {
            perimeter += find(i + 1, j, vec, visited);
        }
    }
    if j > 0 {
        if vec[i][j - 1] == vec[i][j] {
            perimeter += find(i, j - 1, vec, visited);
        }
    }
    if j < vec[0].len() - 1 {
        if vec[i][j + 1] == vec[i][j] {
            perimeter += find(i, j + 1, vec, visited);
        }
    }
    perimeter
}

fn find2(i: usize, j: usize, vec: &Vec<Vec<char>>, visited: &mut Vec<Vec<char>>) -> i32 {
    if visited[i][j] != ' ' {
        return 0;
    }
    visited[i][j] = vec[i][j];
    let mut sides = 0;
    let garden_type = vec[i][j];

    if j == 0 && i == 0 {
        sides += 1;
    }

    if j == 0 && i == vec.len() - 1 {
        sides += 1;
    }

    if j == vec[0].len() - 1 && i == vec.len() - 1 {
        sides += 1;
    }

    if j == vec[0].len() - 1 && i == 0 {
        sides += 1;
    }

    if (j > 0 && i > 0 && vec[i][j - 1] != garden_type && vec[i - 1][j] != garden_type)
        || (j > 0 && i == 0 && vec[i][j - 1] != garden_type)
        || (j == 0 && i > 0 && vec[i - 1][j] != garden_type)
    {
        sides += 1;
    }

    if j < vec[0].len() - 1
        && i < vec.len() - 1
        && vec[i][j + 1] == garden_type
        && vec[i + 1][j] == garden_type
        && vec[i + 1][j + 1] != garden_type
    {
        sides += 1;
    }

    if (j < vec[0].len() - 1
        && i > 0
        && vec[i][j + 1] != garden_type
        && vec[i - 1][j] != garden_type)
        || (j < vec[0].len() - 1 && i == 0 && vec[i][j + 1] != garden_type)
        || (j == vec[0].len() - 1 && i > 0 && vec[i - 1][j] != garden_type)
    {
        sides += 1;
    }

    if j > 0
        && i < vec.len() - 1
        && vec[i][j - 1] == garden_type
        && vec[i + 1][j] == garden_type
        && vec[i + 1][j - 1] != garden_type
    {
        sides += 1;
    }

    if (j > 0 && i < vec.len() - 1 && vec[i][j - 1] != garden_type && vec[i + 1][j] != garden_type)
        || (j > 0 && i == vec.len() - 1 && vec[i][j - 1] != garden_type)
        || (j == 0 && i < vec.len() - 1 && vec[i + 1][j] != garden_type)
    {
        sides += 1;
    }

    if j < vec[0].len() - 1
        && i > 0
        && vec[i][j + 1] == garden_type
        && vec[i - 1][j] == garden_type
        && vec[i - 1][j + 1] != garden_type
    {
        sides += 1;
    }

    if (j < vec[0].len() - 1
        && i < vec.len() - 1
        && vec[i][j + 1] != garden_type
        && vec[i + 1][j] != garden_type)
        || (j < vec[0].len() - 1 && i == vec.len() - 1 && vec[i][j + 1] != garden_type)
        || (j == vec[0].len() - 1 && i < vec.len() - 1 && vec[i + 1][j] != garden_type)
    {
        sides += 1;
    }

    if j > 0
        && i > 0
        && vec[i][j - 1] == garden_type
        && vec[i - 1][j] == garden_type
        && vec[i - 1][j - 1] != garden_type
    {
        sides += 1;
    }

    if i > 0 {
        if vec[i - 1][j] == vec[i][j] {
            sides += find2(i - 1, j, vec, visited);
        }
    }
    if i < vec.len() - 1 {
        if vec[i + 1][j] == vec[i][j] {
            sides += find2(i + 1, j, vec, visited);
        }
    }
    if j > 0 {
        if vec[i][j - 1] == vec[i][j] {
            sides += find2(i, j - 1, vec, visited);
        }
    }
    if j < vec[0].len() - 1 {
        if vec[i][j + 1] == vec[i][j] {
            sides += find2(i, j + 1, vec, visited);
        }
    }
    sides
}

pub fn process_part1(input: &str) -> String {
    let vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![' '; vec[0].len()]; vec.len()];
    let mut perimeter = 0;
    let mut prev_count: HashMap<char, i32> = HashMap::new();

    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            if visited[i][j] == ' ' {
                let p = find(i, j, &vec, &mut visited);
                let mut count = 0;
                for k in 0..visited.len() {
                    for l in 0..visited[0].len() {
                        if visited[k][l] == visited[i][j] {
                            count += 1;
                        }
                    }
                }
                perimeter = perimeter + p * (count - prev_count.get(&visited[i][j]).unwrap_or(&0));
                prev_count
                    .entry(visited[i][j])
                    .and_modify(|e| *e = count)
                    .or_insert(count);
            }
        }
    }
    perimeter.to_string()
}

pub fn process_part2(input: &str) -> String {
    let vec = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut visited = vec![vec![' '; vec[0].len()]; vec.len()];
    let mut sides = 0;
    let mut prev_count: HashMap<char, i32> = HashMap::new();

    for i in 0..vec.len() {
        for j in 0..vec[0].len() {
            if visited[i][j] == ' ' {
                let s = find2(i, j, &vec, &mut visited);
                let mut count = 0;
                for k in 0..visited.len() {
                    for l in 0..visited[0].len() {
                        if visited[k][l] == visited[i][j] {
                            count += 1;
                        }
                    }
                }
                sides = sides + s * (count - prev_count.get(&visited[i][j]).unwrap_or(&0));
                prev_count
                    .entry(visited[i][j])
                    .and_modify(|e| *e = count)
                    .or_insert(count);
            }
        }
    }
    sides.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "1930");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "1206");
    }
}
