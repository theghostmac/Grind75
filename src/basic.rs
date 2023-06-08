fn two_sum(elements: Vec<i32>, target: i32) -> Vec<i32> {
    // create a for loop for first element:
    let i_length = elements.len();
    for i in 0..i_length - 1 {
        // create a for loop for the second element making the pair:
        let j_length = elements.len();
        for j in i + 1..j_length {
            if elements[i] + elements[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    vec![]
}