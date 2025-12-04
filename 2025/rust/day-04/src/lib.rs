
pub fn process_part1(input: &str) -> String {
    let lines = input.lines();
    let vec: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut count = 0;

    for i in 0..vec.len() {
        for j in 0..vec[i].len() {
            if vec[i][j] == '@' {
                let mut around = 0;
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as isize + di;
                        let nj = j as isize + dj;
                        if ni >= 0 && ni < vec.len() as isize && nj >= 0 && nj < vec[i].len() as isize {
                            if vec[ni as usize][nj as usize] == '@' {
                                around += 1;
                            }
                        }
                    }
                }
                if around < 4 {
                    count += 1;
                }
            }
        }
    }


    count.to_string()
}

pub fn process_part2(input: &str) -> String {
    let lines = input.lines();
    let mut vec: Vec<Vec<char>> = lines.map(|line| line.chars().collect()).collect();
    let mut count = 0;
    
    loop {
        let mut ic = 0;

        for i in 0..vec.len() {
            for j in 0..vec[i].len() {
                if vec[i][j] == '@' {
                    let mut around = 0;
                    for di in -1..=1 {
                        for dj in -1..=1 {
                            if di == 0 && dj == 0 {
                                continue;
                            }
                            let ni = i as isize + di;
                            let nj = j as isize + dj;
                            if ni >= 0 && ni < vec.len() as isize && nj >= 0 && nj < vec[i].len() as isize {
                                if vec[ni as usize][nj as usize] == '@' {
                                    around += 1;
                                }
                            }
                        }
                    }
                    if around < 4 {
                        vec[i][j] = '.';
                        ic += 1;
                        count += 1;
                    }
                }
            }
        }
        if ic == 0 {
            break;
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
        assert_eq!(result, "13");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "43");
    }
}
