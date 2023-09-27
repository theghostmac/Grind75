/*
Example 1:

Input: n = 3
Output: ["((()))","(()())","(())()","()(())","()()()"]
Example 2:

Input: n = 1
Output: ["()"]

Constraints:

1 <= n <= 8
 */

// impl Solution {
//     pub fn generate_parenthesis(n: i32) -> Vec<String> {
//         // output is a Vec<String>, create a store for it
//         let mut output = Vec::new();
//         // store for current state of combinations
//         let mut current_state = String::new();
//         Self::generator(&mut output, &mut current_state, n, n);
//         output
//     }
//
//     pub fn generator(
//         output: &mut Vec<String>,
//         current_state: &mut String,
//         left: i32,
//         right: i32) {
//         if left == 0 && right == 0 {
//             output.push(current_state.clone());
//             return;
//         }
//
//         if left > 0 {
//             output.push(String::from('('));
//             Self::generator(output, current_state, left - 1, right);
//             current_state.pop();
//         }
//
//         if right > left {
//             current_state.push(')');
//             Self::generator(output, current_state, left, right - 1);
//             current_state.pop();
//         }
//     }
// }

// Merge into one function
pub fn generate_parenthesis(n: i32) -> Vec<String> {
    fn generator(
        output: &mut Vec<String>,
        current_state: &mut String,
        left: i32,
        right: i32,
    ) {
        if left == 0 && right == 0 {
            output.push(current_state.clone());
            return;
        }

        if left > 0 {
            current_state.push('(');
            generator(output, current_state, left - 1, right);
            current_state.pop();
        }

        if right > left {
            current_state.push(')');
            generator(output, current_state, left, right - 1);
            current_state.pop();
        }
    }

    let mut output = Vec::new();
    let mut current_state = String::new();
    generator(&mut output, &mut current_state, n, n);
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_n4() {
        let n = 4;
        let combinations = generate_parenthesis(n);
        let expected = vec![
            "(((())))",
            "((()()))",
            "((())())",
            "((()))()",
            "(()(()))",
            "(()()())",
            "(()())()",
            "(())(())",
            "(())()()",
            "()((()))",
            "()(()())",
            "()(())()",
            "()()(())",
            "()()()()",
        ];

        // Sort the generated combinations and expected combinations for comparison
        let mut sorted_combinations = combinations.clone();
        sorted_combinations.sort();
        let mut sorted_expected = expected.clone();
        sorted_expected.sort();

        // Check if the generated combinations match the expected combinations
        assert_eq!(sorted_combinations, sorted_expected);
    }
}