use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
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
            right: None,
        }
    }

    pub fn tree_node_wrap(node: TreeNode) -> Option<Rc<RefCell<Self>>> {
        Some(Rc::new(RefCell::new(node)))
    }

    pub fn new_left_right(val: i32, left: i32, right: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: Self::tree_node_wrap(Self::new(right)),
        }
    }

    pub fn new_left(val: i32, left: i32) -> Self {
        TreeNode {
            val,
            left: Self::tree_node_wrap(Self::new(left)),
            right: None,
        }
    }

    pub fn new_right(val: i32, right: i32) -> Self {
        let right = Self::new(right);
        TreeNode {
            val,
            left: None,
            right: Some(Rc::new(RefCell::new(right))),
        }
    }
}

pub struct Solution {}

impl Solution {
    /*
      - refer to question 102 for how lever order traversal works
      - for zigzagging the key is to traverse the entire tree
        either left-to-right or right-to-left depending on the
        lever you are looking for

    */
    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let height = Self::height(&root);

        /*
          - for each iteration you are traversing the tree
            from the top again
          - you will pre-determine how you want to traverse
            the tree based on the level you want to get at
        */
        for i in 1..height + 1 {
            let mut level_result = vec![];
            Self::zigzag_level_order_internal(&root, i, i % 2 == 1, &mut level_result);
            result.push(level_result);
        }
        result
    }

    pub fn zigzag_level_order_internal(
        root: &Option<Rc<RefCell<TreeNode>>>,
        go_down: usize,
        left_to_right: bool,
        level_result: &mut Vec<i32>,
    ) {
        if let Some(refcell) = root {
            let node = refcell.borrow();
            /*
              - you are at the desired level
                and don't need to go down any
                further
              - collect the val into the vec
            */
            if go_down == 1 {
                level_result.push(node.val);
            } else {
                let next_level = go_down - 1;
                if left_to_right {
                    Self::zigzag_level_order_internal(
                        &node.left,
                        next_level,
                        left_to_right,
                        level_result,
                    );
                    Self::zigzag_level_order_internal(
                        &node.right,
                        next_level,
                        left_to_right,
                        level_result,
                    );
                } else {
                    Self::zigzag_level_order_internal(
                        &node.right,
                        next_level,
                        left_to_right,
                        level_result,
                    );
                    Self::zigzag_level_order_internal(
                        &node.left,
                        next_level,
                        left_to_right,
                        level_result,
                    );
                }
            }
        }
    }

    pub fn height(root: &Option<Rc<RefCell<TreeNode>>>) -> usize {
        if let Some(refcell) = root {
            let node = refcell.borrow();
            1 + std::cmp::max(Self::height(&node.left), Self::height(&node.right))
        } else {
            0
        }
    }

    pub fn test_fixture_1() -> Option<Rc<RefCell<TreeNode>>> {
        let r = TreeNode::new_left_right(20, 15, 7);
        let mut root = TreeNode::new_left(3, 9);
        root.right = TreeNode::tree_node_wrap(r);
        TreeNode::tree_node_wrap(root)
    }
    pub fn test_fixture_2() -> Option<Rc<RefCell<TreeNode>>> {
        let root = TreeNode::new(1);
        TreeNode::tree_node_wrap(root)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let target = vec![vec![3], vec![20, 9], vec![15, 7]];
        let tree = Solution::test_fixture_1();
        let result = Solution::zigzag_level_order(tree);

        assert_eq!(result, target);
    }

    #[test]
    fn sample_2() {
        let target = vec![vec![1]];
        let tree = Solution::test_fixture_2();
        let result = Solution::zigzag_level_order(tree);

        assert_eq!(result, target);
    }
    #[test]
    fn sample_3() {
        let target: Vec<Vec<i32>> = vec![];
        let tree = None;
        let result = Solution::zigzag_level_order(tree);
        assert_eq!(result, target);
    }
}
