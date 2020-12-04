fn _translate(c: char) -> char {
    if !c.is_ascii_alphabetic() {
        return c;
    }
    let c = c.to_ascii_lowercase() as u8;
    let z = 'z' as u8;
    let a = 'a' as u8;
    (z - (c - a)) as char
}

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    String::from(
        plain
            .chars()
            .filter(|&c| c.is_ascii_alphanumeric())
            .map(_translate)
            .enumerate()
            .fold(String::new(), |mut res, (i, c)| {
                res.push(c);
                if (i + 1) % 5 == 0 {
                    res.push(' ');
                }
                res
            })
            .trim(),
    )
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|&c| c.is_ascii_alphanumeric() && c != ' ')
        .map(_translate)
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn funtion_translate_should_work() {
        assert_eq!('z', _translate('a'));
        assert_eq!('a', _translate('z'));
        assert_eq!('m', _translate('n'));
        assert_eq!('n', _translate('m'));
    }
}
