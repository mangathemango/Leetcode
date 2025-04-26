/*
 * @lc app=leetcode id=2177 lang=rust
 *
 * [2177] Find Three Consecutive Integers That Sum to a Given Number
 */


// @lc code=start
impl Solution {
    pub fn sum_of_three(num: i64) -> Vec<i64> {
        let mut result = Vec::<i64>::new();
        if num % 3 == 0 {
            for i in (num/3 - 1)..=(num/3 + 1) {
                result.push(i);
            }
        }
        result
    }
}
// @lc code=end

