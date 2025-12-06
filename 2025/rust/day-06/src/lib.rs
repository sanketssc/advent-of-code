pub fn process_part1(input: &str) -> String {
   let lines = input.lines().collect::<Vec<&str>>();
    let mut vec: Vec<Vec<usize>> = vec![vec![]; lines[0].split_whitespace().count()];
    
    for line in lines[0..lines.len() - 1].iter() {
        for (i, num) in line.split_whitespace().enumerate() {
            vec[i].push(num.trim().parse::<usize>().unwrap());
        }
    }
    let mut sum = 0;
    
    let last_line = lines.last().unwrap();
    for (i, op) in last_line.split_whitespace().enumerate() {
        match op {
            "+" => {
                let s: usize = vec[i].iter().sum();
                sum += s;
            }
            "*" => {
                let p: usize = vec[i].iter().product();
                sum += p;
            }
            _ => panic!("Invalid operator"),
        }
    }
    sum.to_string()
}

pub fn process_part2(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let last_line = lines.last().unwrap();
    let data_lines: Vec<&str> = lines[0..lines.len() - 1].iter().cloned().collect();
    
    // Find the maximum width
    let max_width = lines.iter().map(|l| l.len()).max().unwrap();
    
    // Pad all lines to the same width
    let padded_data: Vec<String> = data_lines
        .iter()
        .map(|l| format!("{:width$}", l, width = max_width))
        .collect();
    let padded_last = format!("{:width$}", last_line, width = max_width);
    
    // Process columns right-to-left, grouping by separator columns
    let mut problems: Vec<(Vec<usize>, char)> = vec![];
    let mut current_columns: Vec<Vec<char>> = vec![];
    let mut current_op: Option<char> = None;
    
    for col in (0..max_width).rev() {
        // Check if this column is all spaces (separator)
        let is_separator = padded_data.iter().all(|line| {
            line.chars().nth(col).map_or(true, |c| c == ' ')
        });
        
        let op_char = padded_last.chars().nth(col).unwrap_or(' ');
        
        if is_separator {
            // End of a problem group - save if we have columns
            if !current_columns.is_empty() {
                // Each column becomes a number (read top to bottom)
                let nums: Vec<usize> = current_columns
                    .iter()
                    .map(|col_chars| {
                        let num_str: String = col_chars.iter().filter(|c| c.is_ascii_digit()).collect();
                        if num_str.is_empty() { 0 } else { num_str.parse::<usize>().unwrap() }
                    })
                    .filter(|&n| n > 0)
                    .collect();
                if let Some(op) = current_op {
                    problems.push((nums, op));
                }
                current_columns = vec![];
                current_op = None;
            }
        } else {
            // Collect this column's characters (top to bottom)
            let col_chars: Vec<char> = padded_data.iter().map(|line| {
                line.chars().nth(col).unwrap_or(' ')
            }).collect();
            current_columns.push(col_chars);
            
            // Check for operator
            if op_char == '+' || op_char == '*' {
                current_op = Some(op_char);
            }
        }
    }
    
    // Don't forget the last problem group
    if !current_columns.is_empty() {
        let nums: Vec<usize> = current_columns
            .iter()
            .map(|col_chars| {
                let num_str: String = col_chars.iter().filter(|c| c.is_ascii_digit()).collect();
                if num_str.is_empty() { 0 } else { num_str.parse::<usize>().unwrap() }
            })
            .filter(|&n| n > 0)
            .collect();
        if let Some(op) = current_op {
            problems.push((nums, op));
        }
    }
    
    // Calculate the grand total
    let mut total: usize = 0;
    for (nums, op) in problems {
        let result: usize = match op {
            '+' => nums.iter().copied().sum(),
            '*' => nums.iter().copied().product(),
            _ => panic!("Invalid operator"),
        };
        total += result;
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
        assert_eq!(result, "3");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "6");
    }
}
