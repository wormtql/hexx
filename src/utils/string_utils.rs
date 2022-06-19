pub fn rotate_string(s: &str, count: usize) -> String {
    String::from(&s[count..]) + &s[..count]
}

pub fn substitute_string(s: &str, index: usize, sub: char) -> String {
    s.chars()
        .enumerate()
        .map(|(i, c)| if index == i { sub } else { c })
        .collect()
}

pub fn test_ascii(s: &str, index: usize, v: char) -> bool {
    let x = s.as_bytes()[index];
    x as char == v
}

pub fn invert_string(s: &str, c1: char, c2: char) -> String {
    s.chars()
        .map(|x| if x == c1 { c2 } else if x == c2 { c1 } else { x })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::utils::string_utils::{invert_string, rotate_string};

    #[test]
    fn test_rotate_string1() {
        let s = "123456";
        assert_eq!("234561", rotate_string(s, 1).as_str());
    }

    #[test]
    fn test_invert_string1() {
        let s = "rbrbrrxx";
        assert_eq!("brbrbbxx", invert_string(s, 'r', 'b'));
    }
}