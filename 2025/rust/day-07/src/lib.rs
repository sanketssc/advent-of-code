
pub fn process_part1(input: &str) -> String {
    let vec = input.lines().collect::<Vec<&str>>().into_iter().map(|x| x.chars().collect::<Vec<char>>()).collect::<Vec<Vec<char>>>();
    let mut temp: Vec<bool> = vec[0].iter().map(|&x| x == 'S').collect();
    let size = temp.len();
    let mut splits = 0;
    for line in vec.iter().skip(1) {
        for (i, &ch) in line.iter().enumerate() {
            if ch == '^' && temp[i] {
                splits += 1;
                temp[i] = false;
                temp[i.wrapping_sub(1)] = true;
                temp[i + 1.min(size - 1)] = true;
            }
        }
    }
    splits.to_string()
}

pub fn process_part2(input: &str) -> String {
   let vec = input.lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let size = vec[0].len();
    
    // Track number of paths at each position
    let mut paths: Vec<u64> = vec[0].iter().map(|&x| if x == 'S' { 1 } else { 0 }).collect();
    
    for line in vec.iter().skip(1) {
        let mut next_paths = vec![0u64; size];
        
        for (i, &ch) in line.iter().enumerate() {
            if paths[i] > 0 {
                if ch == '^' {
                    // Split: each path becomes 2 paths (left and right)
                    if i > 0 {
                        next_paths[i - 1] += paths[i];
                    }
                    if i + 1 < size {
                        next_paths[i + 1] += paths[i];
                    }
                } else {
                    // Continue straight down
                    next_paths[i] += paths[i];
                }
            }
        }
        
        paths = next_paths;
    }
    
    // Sum all paths at the end
    paths.iter().sum::<u64>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "21");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "40");
    }
}
