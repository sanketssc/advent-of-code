pub fn process_part1(input: &str) -> String {
    let mut ans = 0;

    for (i, _) in input.chars().into_iter().enumerate() {
        if input.chars().nth(i).unwrap() == 'm' {
            if input.chars().nth(i + 1).unwrap() == 'u' {
                if input.chars().nth(i + 2).unwrap() == 'l' {
                    if input.chars().nth(i + 3).unwrap() == '(' {
                        let mut j = i + 4;
                        let mut num1 = 0;
                        let mut num2 = 0;
                        while input.chars().nth(j).unwrap().is_digit(10) {
                            num1 = num1 * 10 + input.chars().nth(j).unwrap().to_digit(10).unwrap();
                            j += 1;
                        }

                        if input.chars().nth(j).unwrap() != ',' {
                            continue;
                        }
                        j += 1;

                        while input.chars().nth(j).unwrap().is_digit(10) {
                            num2 = num2 * 10 + input.chars().nth(j).unwrap().to_digit(10).unwrap();
                            j += 1;
                        }

                        if input.chars().nth(j).unwrap() != ')' {
                            continue;
                        }

                        ans += num1 * num2;
                    }
                }
            }
        }
    }

    ans.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut ans = 0;

    let mut enable = true;

    for (i, _) in input.chars().into_iter().enumerate() {
        if input.chars().nth(i).unwrap() == 'd' {
            if input.chars().nth(i + 1).unwrap() == 'o' {
                if input.chars().nth(i + 2).unwrap() == 'n' {
                    if input.chars().nth(i + 3).unwrap() == '\'' {
                        if input.chars().nth(i + 4).unwrap() == 't' {
                            if input.chars().nth(i + 5).unwrap() == '(' {
                                enable = false;
                            }
                        }
                    }
                }
            }
        }

        if input.chars().nth(i).unwrap() == 'd' {
            if input.chars().nth(i + 1).unwrap() == 'o' {
                if input.chars().nth(i + 2).unwrap() == '(' {
                    enable = true;
                }
            }
        }

        if !enable {
            continue;
        }
        if input.chars().nth(i).unwrap() == 'm' {
            if input.chars().nth(i + 1).unwrap() == 'u' {
                if input.chars().nth(i + 2).unwrap() == 'l' {
                    if input.chars().nth(i + 3).unwrap() == '(' {
                        let mut j = i + 4;
                        let mut num1 = 0;
                        let mut num2 = 0;
                        while input.chars().nth(j).unwrap().is_digit(10) {
                            num1 = num1 * 10 + input.chars().nth(j).unwrap().to_digit(10).unwrap();
                            j += 1;
                        }

                        if input.chars().nth(j).unwrap() != ',' {
                            continue;
                        }
                        j += 1;

                        while input.chars().nth(j).unwrap().is_digit(10) {
                            num2 = num2 * 10 + input.chars().nth(j).unwrap().to_digit(10).unwrap();
                            j += 1;
                        }

                        if input.chars().nth(j).unwrap() != ')' {
                            continue;
                        }

                        ans += num1 * num2;
                    }
                }
            }
        }
    }

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
        assert_eq!(result, "161");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "48");
    }
}
