#[derive(Debug)]
pub struct TreeNode{
    pub text: String,
    pub children: Vec<TreeNode>
}

impl TreeNode{
    pub fn new(text: String) -> TreeNode{
        TreeNode{
            text,
            children: Vec::new()
        }
    }
    pub fn with_children(text: String, children: Vec<TreeNode>) -> TreeNode{
        TreeNode{
            text,
            children
        }
    }
}

