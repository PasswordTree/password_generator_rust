use super::tree_node::TreeNode;


pub trait TreeNodeExt {
    fn leaves(&self) -> Vec<&TreeNode>;
}

impl TreeNodeExt for TreeNode {
    fn leaves(&self) -> Vec<&TreeNode> {
        let mut nodes: Vec<&TreeNode> = Vec::new();

        if self.children.len() == 0 {
            nodes.push(self);
        }

        for i in &self.children {
            nodes.extend(i.leaves());
        }
        nodes
    }
}