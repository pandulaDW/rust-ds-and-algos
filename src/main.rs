mod data_structures;
mod problems;

use data_structures::linked_list;
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
