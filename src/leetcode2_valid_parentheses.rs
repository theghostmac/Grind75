pub fn is_valid(s: String) -> bool {
    let mut stack = "".to_string();

    for ch in s.chars() {
        match ch {
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            ')' | '}' | ']' => {
                if let Some(m) = stack.pop() {
                    if m != ch {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => return false,
        }
    }
    stack.len() == 0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let s: String = String::from("()");
        let result = is_valid(s);
        assert!(result);
    }

    fn example2() {
        let s: String = String::from("()");
        let result = is_valid(s);
        assert!(result);
    }
}