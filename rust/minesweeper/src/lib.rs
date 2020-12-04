pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut res = vec![];
    if minefield.len() != 0 {
        let rows = minefield.len();
        let row_len = minefield[0].len();
        let minefield_matrix: Vec<&[u8]> = minefield.iter()
            .map(|&row| row.as_bytes())
            .collect();

        for (y, row) in minefield_matrix.iter().enumerate() {
            let mut str_row = String::new();

            for (x, &cell) in row.iter().enumerate() {

                match cell {
                    b'*' => str_row.push(cell as char),
                    _ => {
                        let mut bombs = 0;
                        let start_y = if y == 0 { y } else { y - 1};
                        let start_x = if x == 0 { x } else { x - 1 };
                        let end_y = if y + 1 == rows { y } else { y + 1 };
                        let end_x = if x + 1 == row_len { x } else { x + 1 };
                        for cursor_y in start_y..=end_y {
                            for cursor_x in start_x..=end_x {
                                if cursor_x == x && cursor_y == y { continue; }
                                if minefield_matrix[cursor_y][cursor_x] == b'*' {
                                    bombs += 1;
                                }
                            }
                        }
                        if bombs == 0 {
                            str_row.push(' ');
                        } else {
                            str_row += &format!("{}", bombs);
                        }
                    }
                }

            }

            res.push(str_row);

        }

    }
    res
}
