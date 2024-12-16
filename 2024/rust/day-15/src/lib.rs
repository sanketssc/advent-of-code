use std::collections::{HashMap, VecDeque};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Coordinates {
    x: i128,
    y: i128,
}

pub fn process_part1(input: &str) -> String {
    let (grid, movements, start) = get_grid_and_movements(input);
    let final_grid = get_final_grid(grid, &movements, start);
    final_grid.to_string()
}

pub fn process_part2(input: &str) -> String {
    let (grid, movements, _start) = get_grid_and_movements(input);
    let (expanded_grid, other_start) = expand_grid(grid);
    let final_grid = get_final_grid(expanded_grid, &movements, other_start);
    final_grid.to_string()
}

fn get_grid_and_movements(input: &str) -> (Vec<Vec<char>>, String, Coordinates) {
    let mut grid = Vec::new();
    let mut movements = String::new();
    let mut start = Coordinates { x: 0, y: 0 };
    let mut is_movements = false;

    for (index, row) in input.lines().enumerate() {
        if row.is_empty() {
            is_movements = true;
            continue;
        }

        if is_movements {
            movements.push_str(row);
        } else {
            grid.push(row.chars().collect());
            if let Some(pos) = row.find('@') {
                start = Coordinates {
                    x: pos as i128,
                    y: index as i128,
                };
            }
        }
    }

    (grid, movements, start)
}

fn expand_grid(input: Vec<Vec<char>>) -> (Vec<Vec<char>>, Coordinates) {
    let mut final_grid = Vec::new();
    let mut start = Coordinates { x: 0, y: 0 };

    for (j, row) in input.iter().enumerate() {
        let mut final_row = Vec::new();
        for (i, &char) in row.iter().enumerate() {
            match char {
                '.' => final_row.extend(vec!['.', '.']),
                '#' => final_row.extend(vec!['#', '#']),
                'O' => final_row.extend(vec!['[', ']']),
                '@' => {
                    start = Coordinates {
                        x: (i * 2) as i128,
                        y: j as i128,
                    };
                    final_row.extend(vec!['@', '.']);
                }
                _ => {}
            }
        }
        final_grid.push(final_row);
    }

    (final_grid, start)
}

fn process_step(start: Coordinates, grid: &mut Vec<Vec<char>>, step: char) -> Coordinates {
    let direction = match step {
        '^' => Coordinates { x: 0, y: -1 },
        'v' => Coordinates { x: 0, y: 1 },
        '<' => Coordinates { x: -1, y: 0 },
        '>' => Coordinates { x: 1, y: 0 },
        _ => Coordinates { x: 0, y: 0 },
    };

    let new_x = (start.x as isize + direction.x as isize) as i128;
    let new_y = (start.y as isize + direction.y as isize) as i128;

    if !is_valid_step(Coordinates { x: new_x, y: new_y }, grid) {
        return start;
    }

    match grid[new_y as usize][new_x as usize] {
        '.' => {
            grid[start.y as usize][start.x as usize] = '.';
            grid[new_y as usize][new_x as usize] = '@';
            Coordinates { x: new_x, y: new_y }
        }
        '#' => start,
        'O' => {
            let mut box_new_x = new_x;
            let mut box_new_y = new_y;
            while is_valid_step(
                Coordinates {
                    x: box_new_x,
                    y: box_new_y,
                },
                grid,
            ) {
                match grid[box_new_y as usize][box_new_x as usize] {
                    '.' => {
                        grid[start.y as usize][start.x as usize] = '.';
                        grid[new_y as usize][new_x as usize] = '@';
                        grid[box_new_y as usize][box_new_x as usize] = 'O';
                        return Coordinates { x: new_x, y: new_y };
                    }
                    '#' => return start,
                    _ => {}
                }
                box_new_x = (box_new_x as isize + direction.x as isize) as i128;
                box_new_y = (box_new_y as isize + direction.y as isize) as i128;
            }
            start
        }
        '[' | ']' => {
            if move_boxes(Coordinates { x: new_x, y: new_y }, direction, grid) {
                grid[new_y as usize][new_x as usize] = '@';
                grid[start.y as usize][start.x as usize] = '.';
                Coordinates { x: new_x, y: new_y }
            } else {
                start
            }
        }
        _ => start,
    }
}

fn is_valid_step(current: Coordinates, grid: &[Vec<char>]) -> bool {
    current.x < grid[0].len() as i128 && current.y < grid.len() as i128
}

fn get_final_grid(mut grid: Vec<Vec<char>>, movements: &str, mut start: Coordinates) -> i128 {
    for char in movements.chars() {
        start = process_step(start, &mut grid, char);
    }

    let mut count = 0;
    for (j, row) in grid.iter().enumerate() {
        for (i, &char) in row.iter().enumerate() {
            if char == 'O' || char == '[' {
                count += i + j * 100;
            }
        }
    }
    count as i128
}

fn move_boxes(start: Coordinates, direction: Coordinates, grid: &mut Vec<Vec<char>>) -> bool {
    if direction.y == 0 && (direction.x == 1 || direction.x == -1) {
        return move_boxes_horizontally(start, direction, grid);
    }

    if direction.x == 0 && (direction.y == 1 || direction.y == -1) {
        return move_boxes_vertically(start, direction, grid);
    }
    false
}

fn move_boxes_horizontally(
    start: Coordinates,
    direction: Coordinates,
    grid: &mut Vec<Vec<char>>,
) -> bool {
    let new_x = (start.x as isize + direction.x as isize) as i128;
    let new_y = (start.y as isize + direction.y as isize) as i128;

    match grid[new_y as usize][new_x as usize] {
        '#' => false,
        '.' => {
            grid[new_y as usize][new_x as usize] = grid[start.y as usize][start.x as usize];
            grid[start.y as usize][start.x as usize] = '.';
            true
        }
        '[' | ']' => {
            if move_boxes(Coordinates { x: new_x, y: new_y }, direction, grid) {
                grid[new_y as usize][new_x as usize] = grid[start.y as usize][start.x as usize];
                grid[start.y as usize][start.x as usize] = '.';
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn move_boxes_vertically(
    start: Coordinates,
    direction: Coordinates,
    grid: &mut Vec<Vec<char>>,
) -> bool {
    let mut next = VecDeque::new();
    next.push_back(start);

    if grid[start.y as usize][start.x as usize] == ']' {
        next.push_back(Coordinates {
            x: start.x - 1,
            y: start.y,
        });
    } else {
        next.push_back(Coordinates {
            x: start.x + 1,
            y: start.y,
        });
    }

    let mut visited = HashMap::new();
    let mut visited_slice = Vec::new();

    while let Some(process) = next.pop_front() {
        if visited.contains_key(&process) {
            continue;
        }

        visited.insert(process, ());
        visited_slice.push(process);

        let new_x = (process.x as isize + direction.x as isize) as i128;
        let new_y = (process.y as isize + direction.y as isize) as i128;

        match grid[new_y as usize][new_x as usize] {
            '.' => continue,
            '#' => return false,
            ']' => {
                next.push_back(Coordinates { x: new_x, y: new_y });
                next.push_back(Coordinates {
                    x: new_x - 1,
                    y: new_y,
                });
            }
            '[' => {
                next.push_back(Coordinates { x: new_x, y: new_y });
                next.push_back(Coordinates {
                    x: new_x + 1,
                    y: new_y,
                });
            }
            _ => {}
        }
    }

    for &coord in visited_slice.iter().rev() {
        let x = (coord.x as isize + direction.x as isize) as i128;
        let y = (coord.y as isize + direction.y as isize) as i128;
        grid[y as usize][x as usize] = grid[coord.y as usize][coord.x as usize];
        grid[coord.y as usize][coord.x as usize] = '.';
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = include_str!("./sample_input1.txt");
    const INPUT2: &str = include_str!("./sample_input2.txt");

    #[test]
    fn test_part1() {
        let result = process_part1(INPUT1);
        assert_eq!(result, "10092");
    }
    #[test]
    fn test_part2() {
        let result = process_part2(INPUT2);
        assert_eq!(result, "9021");
    }
}
