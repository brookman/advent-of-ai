
use std::fs;

fn main() {
    let input_string = fs::read_to_string("input.txt").unwrap();
    let mut result = 0;

    // split the input string into lines and store them in a vector
    let lines: Vec<&str> = input_string.lines().collect();

    // iterate through each line of the input
    for (row, line) in lines.iter().enumerate() {
        // iterate through each character in the line
        for (col, c) in line.chars().enumerate() {
            // check if the current character is a gear ('*')
            if c == '*' {
                let mut adjacent_numbers = Vec::new();

                // search for numbers in the surrounding 8 cells
                for i in -1..=1 {
                    for j in -1..=1 {
                        if i == 0 && j == 0 {
                            continue;
                        }

                        let row_index = (row as isize + i) as usize;
                        let col_index = (col as isize + j) as usize;

                        // check if the indices are within the bounds of the input
                        if row_index < lines.len() && col_index < line.len() {
                            // get the character at the current position
                            let ch = lines[row_index].chars().nth(col_index).unwrap();

                            // check if the character is a digit
                            if ch.is_digit(10) {
                                // find the start and end indices of the number
                                let mut start = col_index;
                                while start > 0 && lines[row_index].chars().nth(start - 1).unwrap().is_digit(10) {
                                    start -= 1;
                                }

                                let mut end = col_index;
                                while end < line.len() && lines[row_index].chars().nth(end + 1).unwrap().is_digit(10) {
                                    end += 1;
                                }

                                // extract the number and add it to the adjacent numbers vector
                                let num: u32 = lines[row_index][start..=end].parse().unwrap();
                                if !adjacent_numbers.contains(&num) {
                                    adjacent_numbers.push(num);
                                }
                            }
                        }
                    }
                }

                // check if there are exactly two adjacent numbers
                if adjacent_numbers.len() == 2 {
                    result += adjacent_numbers[0] * adjacent_numbers[1];
                }
            }
        }
    }

    println!("{result}");
}
