use std::fs;

fn main() {
    let mut matrix = [[0; 20]; 20];
    let contents: String = fs::read_to_string("input.txt").expect("Error reading file");
    let mut row = 0;
    let mut col = 0;
    let mut horizontal_max = -1;
    let mut product = 0;
    for line in contents.lines() {
        for num in line.split_whitespace() {
            matrix[row][col] = num.parse().unwrap();
            if col > 2 {
                product = matrix[row][col] * matrix[row][col - 1] * matrix[row][col - 2] * matrix[row][col - 3];
                if  product > horizontal_max {
                    horizontal_max = product;
                }
            }

            col += 1;
        }

        col = 0;
        row += 1;
    }

    let mut vertical_max = -1;
    for col in 0..20 {
        for row in 0..17 {
            product = matrix[row][col] * matrix[row + 1][col] * matrix[row + 2][col] * matrix[row + 3][col];
            if product > vertical_max {
                vertical_max = product;
            }
        }
    }

    let mut diagonal_max = -1;
    for row in 0..17 {
        for col in 0..17 {
            product = matrix[row][col] * matrix[row + 1][col + 1] * matrix[row + 2][col + 2] * matrix[row + 3][col + 3];
            if product > diagonal_max {
                diagonal_max = product;
            }
        }
    }
    for row in 0..17 {
        for col in 3..20 {
            product = matrix[row][col] * matrix[row + 1][col - 1] * matrix[row + 2][col - 2] * matrix[row + 3][col - 3];
            if product > diagonal_max {
                diagonal_max = product;
            }
        }
    }

    println!("Largest horizontal max: {}", horizontal_max);
    println!("Largest vertical max: {}", vertical_max);
    println!("Largest diagonal max: {}", diagonal_max);
}
