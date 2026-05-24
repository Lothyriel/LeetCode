use std::cell::RefCell;
use std::rc::Rc;

pub fn sum_of_left_leaves(root: Option<Node>) -> i32 {
    let mut sum = 0;

    dfs(root.as_ref(), &mut sum);

    sum
}

fn dfs(node: Option<&Node>, sum: &mut i32) {
    let Some(node) = node else {
        return;
    };

    let node = node.borrow();

    *sum += node
        .left
        .as_ref()
        .filter(|n| n.borrow().left.is_none())
        .filter(|n| n.borrow().right.is_none())
        .map(|n| n.borrow().val)
        .unwrap_or_default();

    dfs(node.left.as_ref(), sum);
    dfs(node.right.as_ref(), sum);
}

type Node = Rc<RefCell<TreeNode>>;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Node>,
    pub right: Option<Node>,
}

impl TreeNode {
    pub fn new(val: i32) -> Option<Node> {
        Some(Rc::new(RefCell::new(TreeNode {
            val,
            left: None,
            right: None,
        })))
    }
}

impl From<TreeNode> for Option<Node> {
    fn from(value: TreeNode) -> Self {
        Some(Rc::new(RefCell::new(value)))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let tree = TreeNode {
            val: 3,
            left: TreeNode::new(9),
            right: TreeNode {
                val: 20,
                left: TreeNode::new(15),
                right: TreeNode::new(7),
            }
            .into(),
        };

        assert_eq!(sum_of_left_leaves(tree.into()), 24);

        let tree = TreeNode::new(1);

        assert_eq!(sum_of_left_leaves(tree), 0);

        let tree = TreeNode {
            val: 1,
            left: TreeNode {
                val: 2,
                left: TreeNode::new(4),
                right: TreeNode::new(5),
            }
            .into(),
            right: TreeNode::new(3),
        };

        assert_eq!(sum_of_left_leaves(tree.into()), 4);
    }
}
