pub fn process_part1(input: &str) -> String {
    let arr = input.split(",").collect::<Vec<_>>();
    let ans = arr
        .iter()
        .map(|val| {
            let mut hash = 0;
            val.chars().for_each(|c| {
                hash += c as usize;
                hash *= 17;
                hash = hash % 256;
            });
            hash
        })
        .collect::<Vec<_>>();
    let final_ans = ans.iter().sum::<usize>();
    final_ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let arr = input.split(",").collect::<Vec<_>>();
    let mut boxes: Vec<Vec<(&str, usize)>> = vec![Vec::new(); 256];
    arr.iter().for_each(|val| {
        // println!("{}", val);
        if val.contains("=") {
            let st = val.split("=").nth(0).unwrap();
            let fl = val.split("=").nth(1).unwrap().parse::<usize>().unwrap();
            let mut hash = 0;
            st.chars().for_each(|c| {
                hash += c as usize;
                hash *= 17;
                hash = hash % 256;
            });

            if boxes[hash].iter().any(|(s, _)| *s == st) {
                boxes[hash].iter_mut().for_each(|(s, f)| {
                    if *s == st {
                        *f = fl;
                    }
                });
            } else {
                boxes[hash].push((st, fl));
            }
        }
        if val.contains("-") {
            let st = val.split("-").nth(0).unwrap();
            let mut to_remove = Vec::new();
            boxes.iter().enumerate().for_each(|(it, b)| {
                b.iter().enumerate().for_each(|(i, (s, _))| {
                    if *s == st {
                        to_remove.push((it, i));
                    }
                });
            });
            to_remove.sort_by(|a, b| b.1.cmp(&a.1));
            for (it, i) in to_remove {
                boxes[it].remove(i);
            }
        }
    });

    let mut final_ans = 0;
    for (i, b) in boxes.iter().enumerate() {
        for (it, (_, f)) in b.iter().enumerate() {
            final_ans = final_ans + ((i + 1) * (it + 1) * f);
        }
    }
    final_ans.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "1320");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "145");
    }
}
