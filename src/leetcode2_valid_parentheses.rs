pub fn is_valid(s: String) -> bool {
    // initialize a stack as a mutable empty string.
    // stack DS is commonly for tasks that involve tracking a sequence of elements.
    let mut stack = "".to_string();

    // iterate characters through the string and use match to match them.
    for ch in s.chars() {
        // do some pattern matching.
        match ch {
            '{' => stack.push('}'),
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            ']' | '}' | ')' => {
                if let Some(matched_char) = stack.pop() {
                    if matched_char != ch {
                        return false;
                    }
                } else {
                    // if stack is empty
                    return false;
                }
            }

            // if the character ch doesn't match any of the above cases, return false.
            _ => return false,
        }
    }
    // check that the stack is emppty
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
