/*
 * @lc app=leetcode id=1011 lang=rust
 *
 * [1011] Capacity To Ship Packages Within D Days
 */

// @lc code=start
impl Solution {
    pub fn days_to_ship(weights: &Vec<i32>, capacity: &i32) -> i32 {
        let mut days = 1;
        let mut current_weight = 0;
        for weight in weights {
            if current_weight + weight > *capacity {
                current_weight = 0;
                days += 1;
            }
            current_weight += weight;
        }
        days
    }
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {

        let mut left: i32 = *weights.iter().max().unwrap();
        let mut right: i32 = weights.iter().sum();
        while left < right {
            let capacity = left + (right - left) / 2;
            if Solution::days_to_ship(&weights, &capacity) <= days {
                right = capacity;
            } else {
                left = capacity + 1;
            }
        }
        left
    }
}
// @lc code=end

