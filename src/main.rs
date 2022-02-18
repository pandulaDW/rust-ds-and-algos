mod misc;
mod stack;

use stack::Stack;
fn main() {
    let mut s: Stack<char> = Stack::new();
    s.push('a');
    s.push('b');
    s.push('c');
    s.push('d');
    s.push('e');

    let mut rev: Vec<char> = Vec::new();

    for _ in 0..s.size() {
        rev.push(s.pop().unwrap());
    }

    println!("{}", s);
    println!("{:?}", rev);
}
