/*
 * @lc app=leetcode id=303 lang=rust
 *
 * [303] Range Sum Query - Immutable
 */

// @lc code=start
struct NumArray {
    dp: Vec<i32>
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let mut nums = nums;
        for i in 1..nums.len(){
            nums[i] += nums[i-1];
        }
        NumArray{dp:nums}
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;
        let mut to_extract = 0;
        if left > 0 {
            to_extract = self.dp[left-1];
        }
        self.dp[right] - to_extract
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
// @lc code=end

