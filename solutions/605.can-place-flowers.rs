/*
 * @lc app=leetcode id=605 lang=rust
 *
 * [605] Can Place Flowers
 */

// @lc code=start
impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;

        if flowerbed.len() == 1 {
            return (flowerbed[0] == 0 && n <= 1) || (flowerbed[0] == 1 && n == 0);
        }

        for i in 0..flowerbed.len() {
            if i == 0 {
                if flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    count += 1;
                }
            } else if i == flowerbed.len() - 1 {
                if flowerbed[i - 1] == 0 && flowerbed[i] == 0 {
                    flowerbed[i] = 1;
                    count += 1;
                }
            } else {
                if flowerbed[i - 1] == 0 && flowerbed[i] == 0 && flowerbed[i + 1] == 0 {
                    flowerbed[i] = 1;
                    count += 1;
                }
            }
        }

        count >= n
    }
}
// @lc code=end
