/*
 * @lc app=leetcode id=2057 lang=rust
 *
 * [2057] Smallest Index With Equal Value
 */

// @lc code=start
impl Solution {
    pub fn smallest_equal(nums: Vec<i32>) -> i32 {
        nums
            .iter()
            .enumerate()
            .find_map(
                |(i, num)| 
                if (i % 10) as i32 == *num 
                    {Some(i as i32)} 
                else 
                    {None}
            )
            .unwrap_or(-1)
    }
}
// @lc code=end

