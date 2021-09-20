/*
 * @lc app=leetcode id=94 lang=rust
 *
 * [94] Binary Tree Inorder Traversal
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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = Vec::new();

        fn traversal(node: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
            if let Some(node) = node {
                traversal(node.borrow().left.clone(), result);
                result.push(node.borrow().val);
                traversal(node.borrow().right.clone(), result);
            };
        };

        traversal(root, &mut res);
        res
    }
}
// @lc code=end
