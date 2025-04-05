/*
 * @lc app=leetcode id=2057 lang=rust
 *
 * [2057] Smallest Index With Equal Value
 */

// @lc code=start
impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        for (i, num) in nums.iter().enumerate() {
            if (i as i32) % 10 == *num {
                return i as i32;
            }
        }
        -1
    }
}
// @lc code=end

