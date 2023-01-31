fn main() {
    println!("Calling `different_types_of_strings()` now!");
    different_types_of_strings();
    println!();

    println!("Calling `data_types_with_copy()` now!");
    data_types_with_copy();
    println!();

    println!("Calling `first_word()` now!");
    let phrase = String::from("One Two Three Four Five!");
    let word = first_word(&phrase);
    println!("The returned word is {}.", word);
    println!();
}

fn different_types_of_strings() {
    let s1 = "Hello, world!"; // String literal, on stack, immutable
    // String literal's are known/hardcoded at compile time, fast but immutable

    let mut s2 = String::from("Hello"); // `String` type, mutable, on heap
    s2.push_str(", world!"); // push literal onto `String` type

    println!("\"{}\" is a string literal.", s1);
    println!("\"{}\" is a `String` type.", s2);
}

fn data_types_with_copy() {
    println!("Here is a list of data types with the `Copy` trait:");
    println!("\t- Integer types (u8, i32, isize, etc).");
    println!("\t- Boolean `bool` type ( true | false ).");
    println!("\t- Floating-point types (f64, f32).");
    println!("\t- Character type (char).");
    println!("\t- Tuples* (only if contained data types have the trait).");
    println!("\t    For example (i32, i32) has `Copy`, (i32, String) does not.");
}

fn first_word(s: &str) -> &str { // &str signifies it's returning a slice type
                                //  &str can also encompass &String, but &String
                                //  cannot encompass &str
    println!("Finding first word in phrase: \n - {}", s);
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}