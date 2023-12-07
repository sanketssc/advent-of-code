use std::{cmp::Ordering, collections::HashMap};

fn get_hand(hand: &str) -> i32 {
    let mut hm: HashMap<char, i32> = HashMap::new();
    for i in hand.chars() {
        if let Some(x) = hm.get_mut(&i) {
            *x = *x + 1;
        } else {
            hm.insert(i, 1);
        }
    }
    let val = match hm.len() {
        1 => 1,
        2 => {
            let mut four = false;
            for (_, &value) in &hm {
                if value == 4 {
                    four = true;
                }
            }
            if four {
                return 2;
            } else {
                return 3;
            }
        }
        3 => {
            let mut three = false;
            for (_, &value) in &hm {
                if value == 3 {
                    three = true;
                }
            }
            if three {
                return 4;
            } else {
                return 5;
            }
        }
        4 => 6,
        5 => 7,
        _ => -1,
    };

    val
}

fn compare_hands(a: &str, b: &str) -> Ordering {
    let mut seq: HashMap<char, i32> = HashMap::new();
    seq.insert('A', 1);
    seq.insert('K', 2);
    seq.insert('Q', 3);
    seq.insert('J', 4);
    seq.insert('T', 5);
    seq.insert('9', 6);
    seq.insert('8', 7);
    seq.insert('7', 8);
    seq.insert('6', 9);
    seq.insert('5', 10);
    seq.insert('4', 11);
    seq.insert('3', 12);
    seq.insert('2', 13);

    for i in 0..a.len() {
        if let Some(x) = seq.get(&(a.as_bytes()[i] as char)) {
            if let Some(y) = seq.get(&(b.as_bytes()[i] as char)) {
                if x != y {
                    return y.cmp(x);
                }
            }
        }
    }

    Ordering::Equal
}

fn get_hand_2(hand: &str) -> i32 {
    let mut hm: HashMap<char, i32> = HashMap::new();
    let mut count_j = 0;
    for i in hand.chars() {
        if i != 'J' {
            if let Some(x) = hm.get_mut(&i) {
                *x = *x + 1;
            } else {
                hm.insert(i, 1);
            }
        } else {
            count_j += 1;
        }
    }

    if let Some(max_entry) = hm.iter().max_by(|a, b| a.1.cmp(&b.1)) {
        let max_val = max_entry.0.clone();
        if let Some(x) = hm.get_mut(&max_val) {
            *x = *x + count_j;
        }
    } else {
        hm.insert('J', count_j);
    }

    let val = match hm.len() {
        1 => 1,
        2 => {
            let mut four = false;
            for (_, &value) in &hm {
                if value == 4 {
                    four = true;
                }
            }
            if four {
                2
            } else {
                3
            }
        }
        3 => {
            let mut three = false;
            for (_, &value) in &hm {
                if value == 3 {
                    three = true;
                }
            }
            if three {
                4
            } else {
                5
            }
        }
        4 => 6,
        5 => 7,
        _ => -1,
    };

    val
}

fn compare_hands_2(a: &str, b: &str) -> Ordering {
    let mut seq: HashMap<char, i32> = HashMap::new();
    seq.insert('A', 1);
    seq.insert('K', 2);
    seq.insert('Q', 3);
    seq.insert('J', 14);
    seq.insert('T', 5);
    seq.insert('9', 6);
    seq.insert('8', 7);
    seq.insert('7', 8);
    seq.insert('6', 9);
    seq.insert('5', 10);
    seq.insert('4', 11);
    seq.insert('3', 12);
    seq.insert('2', 13);

    for i in 0..a.len() {
        if let Some(x) = seq.get(&(a.as_bytes()[i] as char)) {
            if let Some(y) = seq.get(&(b.as_bytes()[i] as char)) {
                if x != y {
                    return y.cmp(x);
                }
            }
        }
    }

    Ordering::Equal
}
pub fn process_part1(input: &str) -> String {
    let mut arr = input
        .lines()
        .map(|line| {
            (
                get_hand(line.split(" ").nth(0).unwrap()),
                line.split(" ").nth(0).unwrap(),
                line.split(" ").nth(1).unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    arr.sort_by(|a, b| {
        if a.0 == b.0 {
            return compare_hands(a.1, b.1);
        }
        b.0.cmp(&a.0)
    });
    let mut sol = 0;

    for i in 1..=arr.len() {
        sol = sol + (arr[i - 1].2 * i as i64);
    }

    sol.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut arr = input
        .lines()
        .map(|line| {
            (
                get_hand_2(line.split(" ").nth(0).unwrap()),
                line.split(" ").nth(0).unwrap(),
                line.split(" ").nth(1).unwrap().parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    arr.sort_by(|a, b| {
        if a.0 == b.0 {
            return compare_hands_2(a.1, b.1);
        }
        b.0.cmp(&a.0)
    });
    let mut sol = 0;

    for i in 1..=arr.len() {
        sol = sol + (arr[i - 1].2 * i as i64);
    }

    sol.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "6440");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "5905");
    }
}
