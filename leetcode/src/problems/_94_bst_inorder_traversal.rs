/*
Problem: Given the root of a binary tree, return the inorder traversal of its nodes' values.
*/

struct Solution;
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        // response vector, on each visited node, values discovered in traversal will be pushed to here
        let mut res: Vec<i32> = Vec::new();
        // stack to track nodes we still have to visit
        let mut stack = Vec::new();
        // working copy of tree passed into function
        let mut r = root.clone();
        // while working copy of tree still has nodes left to traverse
        while r.is_some() || !stack.is_empty() {
            // if current iteration yields a node, push it to the stack, go left for inorder traversal
            while let Some(node) = r {
                stack.push(node.clone());
                r = node.borrow().left.clone();
            }
            // once we run of out left children nodes to visit, start working on the stack we just created.
            r = stack.pop();
            // will start with deepest left child, and collect the value of the node by pushing to response vec
            if let Some(node) = r {
                res.push(node.borrow().val);
                // since we are on deepest left node, now it's okay to check for right children, update r and continue traversal
                r = node.borrow().right.clone();
            }
        }
        res
    }
}