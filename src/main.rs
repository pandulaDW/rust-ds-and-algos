mod binary_tree;
mod easy;
mod linked_list;
mod medium;
mod stack;
mod vec;

fn main() {
    use linked_list::List;

    let mut list: List<i32> = List::new();
    list.append(12);
    list.append(152);
    list.append(32);
    list.append(129);
    list.append(12444);

    println!("{}", list);
}
