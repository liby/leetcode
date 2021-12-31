/*
 * @lc app=leetcode id=700 lang=rust
 *
 * [700] Search in a Binary Search Tree
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
use std::cmp::Ordering;
use std::rc::Rc;
impl Solution {
    pub fn search_bst(
        root: Option<Rc<RefCell<TreeNode>>>,
        val: i32,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let mut node = root;
        while let Some(rc_node) = node.clone() {
            let cur_node = rc_node.borrow();
            match cur_node.val.cmp(&val) {
                Ordering::Equal => return node,
                Ordering::Less if cur_node.right.is_some() => node = cur_node.right.clone(),
                Ordering::Greater if cur_node.left.is_some() => node = cur_node.left.clone(),
                _ => break,
            }
        }
        None
    }
}
// @lc code=end
