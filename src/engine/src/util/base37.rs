pub fn decode37(mut value: u64) -> String {
    if value == 0 {
        return String::from("_");
    }

    let mut chars = Vec::new();

    while value != 0 {
        let remainder = (value % 37) as u8;
        value /= 37;

        let c = match remainder {
            1..=26 => (b'a' + remainder - 1) as char,
            27..=36 => (b'0' + remainder - 27) as char,
            _ => '_',
        };

        chars.push(c);
    }

    chars.reverse(); // Since we decoded in reverse
    chars.iter().collect()
}

pub fn encode37(string: &str) -> Option<i64> {
    if string.is_empty()  ||string.contains(' ') {
        return None;
    }

    let mut result: i64 = 0;
    for c in string.chars() {
        let value = match c {
            'a'..='z' => (c as u8 - b'a' + 1) as i64,
            '0'..='9' => (c as u8 - b'0' + 27) as i64,
            _ => return None,
        };
        result = result * 37 + value;
    }
    Some(result)
}