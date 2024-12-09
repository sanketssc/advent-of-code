pub fn process_part1(input: &str) -> String {
    let mut vals = vec![];
    let mut free = false;
    let mut id = 0;
    for len in input.chars() {
        match free {
            true => {
                for _ in 0..len.to_digit(10).unwrap() {
                    vals.push(None);
                }
            }
            false => {
                for _ in 0..len.to_digit(10).unwrap() {
                    vals.push(Some(id));
                }
                id += 1;
            }
        }
        free = !free;
    }

    let mut i = 0;
    'outer: loop {
        while vals.last().unwrap().is_none() {
            vals.pop().unwrap();
        }
        let to_move = vals.pop().unwrap();
        while {
            if i >= vals.len() {
                vals.push(to_move);
                break 'outer;
            }
            vals[i].is_some()
        } {
            i += 1;
        }
        vals[i] = to_move;
    }

    let ans = vals
        .iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap_or(0))
        .sum::<usize>();
    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut vals = vec![];
    let mut free = false;
    let mut id = 0;
    for len in input.chars() {
        match free {
            true => {
                for _ in 0..len.to_digit(10).unwrap() {
                    vals.push(None);
                }
            }
            false => {
                for _ in 0..len.to_digit(10).unwrap() {
                    vals.push(Some(id));
                }
                id += 1;
            }
        }
        free = !free;
    }
    for move_id in (0..=id).rev() {
        let len = vals.iter().filter(|x| **x == Some(move_id)).count();
        for i in 0..vals.len() {
            if vals[i] == Some(move_id) {
                break;
            }
            if (i..(i + len)).all(|x| vals[x].is_none()) {
                for x in vals.iter_mut() {
                    if *x == Some(move_id) {
                        *x = None;
                    }
                }
                vals[i..(i + len)].fill(Some(move_id));
                break;
            }
        }
    }

    let ans = vals
        .iter()
        .enumerate()
        .map(|(i, x)| i * x.unwrap_or(0))
        .sum::<usize>();
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
        assert_eq!(result, "1928");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "2858");
    }
}
