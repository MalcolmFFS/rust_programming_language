fn main() {
    //let mut s = String::from("hello");

    let s = String::from("Hello from the other side.");

    println!("{}", first_word(&s))
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
