use tree::BinaryTree;

fn main() {
    let mut bt = BinaryTree::new('a');
    let root = bt.get_key();
    println!("root val is {:?}", root);

    let left = bt.get_left();
    println!("left child in {:#?}", left);

    let right = bt.get_right();
    println!("left child in {:#?}", right);

    bt.insert_left_tree('b');
    bt.insert_left_tree('e');

    let left = bt.get_left();
    println!("left child is {:#?}", left);

    let right = bt.get_right();
    println!("right child is {:#?}", right);
}
