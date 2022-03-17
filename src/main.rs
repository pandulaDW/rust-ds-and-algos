mod data_structures;
mod problems;
mod recursion;

use data_structures::binary_tree::BinaryTree;
fn main() {
    let mut tree = BinaryTree::Empty;
    tree.add("jaeger");
    tree.add("robot");
    tree.add("droid");
    tree.add("mecha");

    for v in tree.into_iter() {
        println!("{:?}", v);
    }
}
