use std::io;

fn peek_char(s: (&str, usize)) -> Option<char> {
    match s.0.chars().nth(s.1) {
        None => None,
        Some(ch) => Some(ch)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);

    println!("{:?}", peek_char((&*input, 1)).unwrap());
    println!("{:?}", peek_char((&*input, 2)).unwrap());
    println!("{:?}", peek_char((&*input, 200)).unwrap());
}
