// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    // check rows count
    if lines.len() % 4 != 0 {
        return Err(Error::InvalidRowCount(lines.len()));
    }
    // check columns len
    for line in &lines {
        if line.len() % 3 != 0 {
            return Err(Error::InvalidColumnCount(line.len()));
        }
    }

    let blocks = lines
        .iter()
        .map(|line| line.as_bytes().chunks(3).collect::<Vec<&[u8]>>())
        .collect::<Vec<Vec<&[u8]>>>();

    let mut result = String::new();
    for block in blocks.chunks(4) {
        for index in 0..block[0].len() {
            // create the digit array
            let char_n: [&[u8]; 4] = [
                block[0][index],
                block[1][index],
                block[2][index],
                block[3][index],
            ];
            // match digit array with digits map
            result.push(match char_n {
                c if c == DIGITS_MAP[0] => '0',
                c if c == DIGITS_MAP[1] => '1',
                c if c == DIGITS_MAP[2] => '2',
                c if c == DIGITS_MAP[3] => '3',
                c if c == DIGITS_MAP[4] => '4',
                c if c == DIGITS_MAP[5] => '5',
                c if c == DIGITS_MAP[6] => '6',
                c if c == DIGITS_MAP[7] => '7',
                c if c == DIGITS_MAP[8] => '8',
                c if c == DIGITS_MAP[9] => '9',
                _ => '?',
            });
        }
        result.push(',');
    }
    result.pop();
    Ok(result)
}

#[cfg_attr(rustfmt, rustfmt_skip)]
const DIGITS_MAP: [[&[u8]; 4]; 10] = [
    [
        b" _ ",
        b"| |",
        b"|_|",
        b"   ",
    ],
    [
        b"   ",
        b"  |",
        b"  |",
        b"   ",
    ],
    [
        b" _ ",
        b" _|",
        b"|_ ",
        b"   "
    ],
    [
        b" _ ",
        b" _|",
        b" _|",
        b"   "
    ],
    [
        b"   ",
        b"|_|",
        b"  |",
        b"   "
    ],
    [
        b" _ ",
        b"|_ ",
        b" _|",
        b"   "
    ],
    [
        b" _ ",
        b"|_ ",
        b"|_|",
        b"   "
    ],
    [
        b" _ ",
        b"  |",
        b"  |",
        b"   "
    ],
    [
        b" _ ",
        b"|_|",
        b"|_|",
        b"   "
    ],
    [
        b" _ ",
        b"|_|",
        b" _|",
        b"   "
    ],
];
