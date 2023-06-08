pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for i in 0..nums.len() -1 {
        for j in i + 1..nums.len() {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32]
            }
        }
    }
    vec![]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let two_sum_result = two_sum(vec![2,7,11,15], 9);
        assert_eq!(two_sum_result, vec![0,1]);
    }
}