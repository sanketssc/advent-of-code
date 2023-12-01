pub fn process_part1(input: &str) -> String {
    let res1 = input
        .lines()
        .map(|word| {
            for (_i, ch) in word.char_indices() {
                if ch.is_numeric() {
                    return (ch as i32 - 48) * 10;
                }
            }
            -1
        })
        .collect::<Vec<_>>();
    let res2 = input
        .lines()
        .map(|word| {
            for (_i, ch) in word.char_indices().rev() {
                if ch.is_numeric() {
                    return ch as i32 - 48;
                }
            }
            -1
        })
        .collect::<Vec<_>>();
    let mut ans_vec = Vec::<i32>::new();
    for i in 0..res1.len() {
        let sum = res1[i] + res2[i];
        ans_vec.push(sum);
    }
    let ans: i32 = ans_vec.iter().sum();

    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let pat = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let r = input
        .lines()
        .map(|line| {
            let mut matching_words: Vec<(usize, usize)> = Vec::new();
            for (id, &patt) in pat.iter().enumerate() {
                //find from left
                if let Some(index) = line.find(patt) {
                    matching_words.push((index, id));
                }
                //find from right
                if let Some(index) = line.rfind(patt) {
                    matching_words.push((index, id));
                }
            }
            for (i, ch) in line.char_indices() {
                if ch.is_numeric() {
                    matching_words.push((i, ch as usize - 48));
                }
            }
            matching_words.sort_by(|a, b| a.0.cmp(&b.0));
            println!(
                "{:?}",
                matching_words.first().unwrap().1 * 10 + matching_words.last().unwrap().1
            );

            matching_words.first().unwrap().1 * 10 + matching_words.last().unwrap().1
        })
        .sum::<usize>();
    println!("{}", r);
    r.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "142");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "21");
    }
}
