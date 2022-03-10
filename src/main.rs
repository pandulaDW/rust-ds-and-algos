mod data_structures;
mod problems;
mod recursion;

fn main() {
    let mut v = vec!["foo", "bar", "baz", "dawn", "fam", "dam"];
    problems::sorting::_selection_sort(&mut v);
    println!("{:?}", v);
}
