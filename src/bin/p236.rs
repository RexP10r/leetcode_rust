#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}
impl TreeNode {
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

struct Solution;

impl Solution {
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        };
        let (left_child, right_child) = {
            let node = root.as_ref().unwrap().borrow();
            (node.left.clone(), node.right.clone())
        };
        let left = Self::lowest_common_ancestor(left_child, p.clone(), q.clone());
        if left.is_some() && left != p && left != q {
            return left;
        };
        let right = Self::lowest_common_ancestor(right_child, p.clone(), q.clone());
        if left.is_some() && right.is_some() {
            return root;
        };
        left.or(right)
    }
}

fn main() {
    let _tmp = Solution::lowest_common_ancestor(None, None, None);
}
