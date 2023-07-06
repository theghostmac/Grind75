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
                        // if stack is empty.
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
       assert!(is_valid("()".to_string()))
    }

    #[test]
    fn example2() {
        assert!(is_valid("()[]{}".to_string()))
    }

    #[test]
    fn example3() {
        assert!(!is_valid("(]".to_string()))
    }

    #[test]
    fn example4() {
        assert!(is_valid("".to_string()))
    }
}