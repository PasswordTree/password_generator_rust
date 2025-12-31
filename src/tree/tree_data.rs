use crate::tree::tree_node::TreeNode;

pub fn data() -> TreeNode{
    TreeNode::with_children(String::from("root"),vec![
        TreeNode::with_children(String::from("Letter"),vec![
            TreeNode::with_children(String::from("Latin"),vec![
                TreeNode::with_children(String::from("ASCII"),vec![
                    TreeNode::with_children(String::from("Lowercase"),vec![
                        TreeNode::new(String::from("abcdefghijklmnopqrstuvwxyz"))
                    ]),
                    TreeNode::with_children(String::from("Uppercase"),vec![
                        TreeNode::new(String::from("ABCDEFGHIJKLMNOPQRSTUVWXYZ"))
                    ]),
                ])
            ])
        ]),
        TreeNode::with_children(String::from("Number"),vec![
            TreeNode::with_children(String::from("ASCII"),vec![
                TreeNode::new(String::from("0123456789"))
            ]),
            TreeNode::with_children(String::from("Non-ASCII"),vec![
                TreeNode::new(String::from("٠١٢٣٤٥٦٧٨٩۰۱۲۳۴۵۶۷۸۹"))
            ])
        ]),
        TreeNode::with_children(String::from("Symbol"),vec![
            TreeNode::new(String::from("!\"#$%&\'()*+,-./:;<=>?@[\\]^_`{|}~"))
        ])
    ])
}