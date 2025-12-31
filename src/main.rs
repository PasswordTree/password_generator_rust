mod tree;
mod password;

use crate::tree::tree_data;
use crate::password::Password;

fn main() {
    let my_tree = tree_data::data();

    let mut password = Password::new(my_tree);
    password.distinct = true;
    
    for _number in 1..=4 {
        match password.generate(30) { 
            Ok(pass) => println!("{}", pass),
            Err(err) => println!("{}", err),
        }
    }
}
