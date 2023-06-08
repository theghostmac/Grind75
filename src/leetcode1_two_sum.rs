pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // start a for loop, iterate over 0 to nums.len() - 1.
    for i in 0..nums.len() - 1 {
        // start another loop that iterates over i+1 to nums.len().
        for j in i + 1..nums.len() {
            // if two values in the loops sums up to target value, make a vector from them.
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    // else, create and return an empty vector.
    vec![]
}

// using arrays instead of Vectors.
pub fn two_sum_with_arrays(elements: &[i32], target: i32) -> Option<(usize, usize)> {
    for i in 0..elements.len() {
        for j in i + 1..elements.len() {
            if elements[i] + elements[j] == target {
                return Some((i, j));
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let two_sum_result = two_sum(vec![2,7,11,15], 9);
        assert_eq!(two_sum_result, vec![0,1]);
    }

    #[test]
    fn example2() {
        let two_sum_result = two_sum(vec![3,2,4], 6);
        assert_eq!(two_sum_result, vec![1,2]);
    }

    #[test]
    fn example3() {
        let two_sum_result = two_sum(vec![3,3], 6);
        assert_eq!(two_sum_result, vec![0,1]);
    }

    #[test]
    fn array_experiment() {
        let test_array = [2,7,11,15];
        let two_sum_with_array_result = two_sum_with_arrays(&test_array, 9);
        assert_eq!(two_sum_with_array_result, Some((0,1)));
    }

}

