mod data_structures;
mod problems;
mod recursion;

fn main() {
    let s = "peekaboo";
    let mut iter = s.as_bytes().iter().peekable();

    loop {
        if iter.peek().is_none() {
            break;
        }

        match iter.next().unwrap() {
            b'e' => println!("?"),
            v => println!("{}", v),
        }
    }
}
