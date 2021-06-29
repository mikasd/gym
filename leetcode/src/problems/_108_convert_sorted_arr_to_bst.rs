// Definition for a binary tree node.
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
struct Solution;
impl Solution {
    fn make_tree(arr:&Vec<i32>,left:usize,right:usize) -> Option<Rc<RefCell<TreeNode>>> { 
        if left>=right
        {
            return None;
        }
        let mid = left + (right-left) / 2;
        let root = Some(Rc::new(RefCell::new(TreeNode{
            val: arr[mid as usize],
            left:  Self::make_tree(&arr, left, mid),
            right: Self::make_tree(&arr, mid+1,right),
        })));
        root
    }
          
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        return Self::make_tree(&nums,0,nums.len());
    }
}