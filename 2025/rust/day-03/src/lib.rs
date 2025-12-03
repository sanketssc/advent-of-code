pub fn process_part1(input: &str) -> String {
    // println!("input: {}", input.lines().count());
    let mut i = 0;
    let ans = input.lines().fold(0, |acc, line| {
        i += 1;
        println!("line: {}", i);
        let line_nums: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let mut max = line_nums[0];
        let mut maxidx = 0;
        for (i, &num) in line_nums.iter().enumerate() {
            if num > max {
                max = num;
                maxidx = i;
            }
        }

        println!("len: {}, maxidx: {}", line_nums.len(), maxidx);
        let mut max2 = 0;
        let mut maxidx2 = 0;
        if maxidx < line_nums.len() -1 {
            for (i, &num) in line_nums.iter().enumerate().skip(maxidx + 1) {
                if num > max2 {
                    max2 = num;
                    maxidx2 = i;
                }
            }
        }else {
            for (i, &num) in line_nums.iter().enumerate().take(maxidx)
            {
                if num > max2 {
                    max2 = num;
                    maxidx2 = i;
                }
            }
        }

        if maxidx < maxidx2 {
            println!("{}", max*10 + max2);
            acc + max * 10 + max2
        } else {
            println!("{}", max2*10 + max);
            acc + max2 * 10 + max
        }



    });
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut i = 0;
    let ans = input.lines().fold(0, |acc, line| {
        i += 1;
        let line_digits: Vec<char> = line.chars().collect();
        let n = line_digits.len();
        let k = 12; // number of digits we need to pick
        
        let mut subseq = Vec::new();
        let mut start = 0;
        
        for remaining in (1..=k).rev() {
            let end = n - remaining;
            
            let mut max_char = '0';
            let mut max_idx = start;
            
            for (idx, &c) in line_digits.iter().enumerate().skip(start).take(end - start + 1) {
                if c > max_char {
                    max_char = c;
                    max_idx = idx;
                }
            }
            
            subseq.push(max_char);
            start = max_idx + 1;
        }
        
        let subseq_str: String = subseq.into_iter().collect();
        println!("subseq: {}", subseq_str);
        acc + subseq_str.parse::<i64>().unwrap()
    });
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
        assert_eq!(result, "357");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "3121910778619");
    }
}
