/*
 * @lc app=leetcode id=541 lang=rust
 *
 * [541] Reverse String II
 */

// @lc code=start
impl Solution {
    pub fn reverse_str(s: String, k: i32) -> String {
        let mut v = s.chars().map(|x| x.to_string()).collect::<Vec<String>>();
        let L = s.len() - 1;
        let k = k as usize;

        let mut i = 0;

        while i * k <= L {
            let mut n = i * k;
            let mut m = L.min((i + 1) * k - 1);

            while n < m {
                v.swap(m, n);
                n += 1;
                m -= 1;
            }

            i += 2;
        }

        v.join("")
    }
}
// @lc code=end
