fn find_xmas(matrix: Vec<&str>, i: usize, j: usize) -> i32 {
    let mut count = 0;

    if j >= 3 {
        if matrix[i].chars().nth(j - 1) == Some('M')
            && matrix[i].chars().nth(j - 2) == Some('A')
            && matrix[i].chars().nth(j - 3) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    if i >= 3 {
        if matrix[i - 1].chars().nth(j) == Some('M')
            && matrix[i - 2].chars().nth(j) == Some('A')
            && matrix[i - 3].chars().nth(j) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    if j < matrix[i].len() - 3 {
        if matrix[i].chars().nth(j + 1) == Some('M')
            && matrix[i].chars().nth(j + 2) == Some('A')
            && matrix[i].chars().nth(j + 3) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    if i < matrix.len() - 3 {
        if matrix[i + 1].chars().nth(j) == Some('M')
            && matrix[i + 2].chars().nth(j) == Some('A')
            && matrix[i + 3].chars().nth(j) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    if i < matrix.len() - 3 && j < matrix[i].len() - 3 {
        if matrix[i + 1].chars().nth(j + 1) == Some('M')
            && matrix[i + 2].chars().nth(j + 2) == Some('A')
            && matrix[i + 3].chars().nth(j + 3) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    if i < matrix.len() - 3 && j >= 3 {
        if matrix[i + 1].chars().nth(j - 1) == Some('M')
            && matrix[i + 2].chars().nth(j - 2) == Some('A')
            && matrix[i + 3].chars().nth(j - 3) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    if i >= 3 && j < matrix[i].len() - 3 {
        if matrix[i - 1].chars().nth(j + 1) == Some('M')
            && matrix[i - 2].chars().nth(j + 2) == Some('A')
            && matrix[i - 3].chars().nth(j + 3) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    if i >= 3 && j >= 3 {
        if matrix[i - 1].chars().nth(j - 1) == Some('M')
            && matrix[i - 2].chars().nth(j - 2) == Some('A')
            && matrix[i - 3].chars().nth(j - 3) == Some('S')
            && matrix[i].chars().nth(j) == Some('X')
        {
            count += 1;
        }
    }

    count
}

pub fn process_part1(input: &str) -> String {
    let mut count = 0;

    let vec = input.lines().collect::<Vec<_>>();

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i].chars().nth(j) == Some('X') {
                count += find_xmas(vec.clone(), i, j);
            }
        }
    }

    count.to_string()
}

fn find_mas_mas(matrix: Vec<&str>, i: usize, j: usize) -> i32 {
    let mut count = 0;

    if j < 1 || j > matrix[i].len() - 2 {
        return 0;
    }

    if i < 1 || i > matrix.len() - 2 {
        return 0;
    }

    if matrix[i - 1].chars().nth(j + 1) == Some('M')
        && matrix[i - 1].chars().nth(j - 1) == Some('M')
        && matrix[i + 1].chars().nth(j - 1) == Some('S')
        && matrix[i + 1].chars().nth(j + 1) == Some('S')
    {
        count += 1;
    }

    if matrix[i - 1].chars().nth(j + 1) == Some('S')
        && matrix[i - 1].chars().nth(j - 1) == Some('S')
        && matrix[i + 1].chars().nth(j - 1) == Some('M')
        && matrix[i + 1].chars().nth(j + 1) == Some('M')
    {
        count += 1;
    }

    if matrix[i - 1].chars().nth(j + 1) == Some('S')
        && matrix[i - 1].chars().nth(j - 1) == Some('M')
        && matrix[i + 1].chars().nth(j - 1) == Some('M')
        && matrix[i + 1].chars().nth(j + 1) == Some('S')
    {
        count += 1;
    }

    if matrix[i - 1].chars().nth(j + 1) == Some('M')
        && matrix[i - 1].chars().nth(j - 1) == Some('S')
        && matrix[i + 1].chars().nth(j - 1) == Some('S')
        && matrix[i + 1].chars().nth(j + 1) == Some('M')
    {
        count += 1;
    }

    count
}

pub fn process_part2(input: &str) -> String {
    let mut count = 0;

    let vec = input.lines().collect::<Vec<_>>();

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i].chars().nth(j) == Some('A') {
                count += find_mas_mas(vec.clone(), i, j);
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
        assert_eq!(result, "18");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "9");
    }
}
