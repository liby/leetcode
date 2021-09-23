/*
 * @lc app=leetcode id=104 lang=rust
 *
 * [104] Maximum Depth of Binary Tree
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let mut max_val = 0;

        if let Some(node) = root {
            let left = 1 + Self::max_depth(node.borrow_mut().left.take());
            let right = 1 + Self::max_depth(node.borrow_mut().right.take());

            max_val = std::cmp::max(left, right);
        }

        max_val
    }
}
// @lc code=end
