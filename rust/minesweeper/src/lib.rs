use std::char;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res = vec![];
    if !minefield.is_empty() {
        let rows = minefield.len();
        let row_len = minefield[0].len();
        let minefield_matrix: Vec<&[u8]> = minefield.iter().map(|&row| row.as_bytes()).collect();

        for (y, row) in minefield_matrix.iter().enumerate() {
            let mut str_row = String::new();

            for (x, &cell) in row.iter().enumerate() {
                match cell {
                    b'*' => str_row.push(cell as char),
                    _ => {
                        let mut bombs = 0;
                        let start_y = y.saturating_sub(1);
                        let start_x = x.saturating_sub(1);
                        let end_y = if y + 1 == rows { y } else { y + 1 };
                        let end_x = if x + 1 == row_len { x } else { x + 1 };
                        for cursor_y in start_y..=end_y {
                            for cursor_x in start_x..=end_x {
                                if cursor_x == x && cursor_y == y {
                                    continue;
                                }
                                if minefield_matrix[cursor_y][cursor_x] == b'*' {
                                    bombs += 1;
                                }
                            }
                        }
                        str_row.push(if bombs == 0 {
                            ' '
                        } else {
                            char::from_digit(bombs, 10).unwrap()
                        })
                    }
                }
            }

            res.push(str_row);
        }
    }
    res
}
