use rand::Rng;
use crate::tree::tree_node::TreeNode;
use crate::tree::tree_extension::TreeNodeExt;

#[derive(Debug)]
pub struct Password {
    pub distinct: bool,
    pub tree: TreeNode
}

impl Password {
    pub fn new(tree: TreeNode) -> Self {
        Password {
            distinct: false,
            tree
        }
    }

    pub fn generate(&self, requested_length: u32) -> Result<String, String> {
        let mut password = String::from("");
        let pruned_leaves = self.tree.leaves();

        let possible_password_length: u32 = self.get_possible_password_length();
        if requested_length > possible_password_length {
            return Err(format!(
                "Sum of all selected leaves is less than password length\n\
                Max length based on selected nodes: {}",
                possible_password_length
            ));
        }

        while password.len() < requested_length as usize {
            let random_leaf_index = rand::thread_rng().gen_range(0..pruned_leaves.len());
            let selected_leaf : &TreeNode = pruned_leaves[random_leaf_index];

                let tree_value: Vec<char> = selected_leaf.text.chars().collect();
                let selected_char = tree_value[rand::thread_rng().gen_range(0..tree_value.len())];

                if !(self.distinct && password.contains(selected_char)){
                    password.push(selected_char);
                }

        }
        Ok(password)
    }
    
    fn get_possible_password_length(&self) -> u32{
        let tmp = self.tree.leaves();
        let mut length: u32 = 0;
        
        for i in &tmp{
            length += i.text.len() as u32;
        }
        length
    }
}
