fn main() {
    vectors();
    println!();
    strings();
    println!();
    hashmap();
    println!();
}

fn vectors() {
    // Building a vector starting wtih no elements requires declaring type.
    let mut v1: Vec<i32> = Vec::new();

    v1.push(5);
    v1.push(6);
    v1.push(7);
    v1.push(8);

    // Grab a value from vector if it exists, otherwise crash. Not error friendly.
    let third: &i32 = &v1[2];
    println!("The third element of v1 is {third}.");

    // Building a vector with elements and vec! macro, it infers typing from elements.
    let v2 = vec![1, 2, 3, 4, 5];

    // Using Vector.get(idx) returns a Option<T>, and requires `match` to process it further.
    let third: Option<&i32> = v2.get(2);
    match third {
        Some(third) => println!("The third element of v2 is {third}."),
        None => println!("There is no third element in v2."),
    }

    // NOTE: when borrowing an immutable reference to a vector, the entire vector becomes immmutable
    //       and cannot be mutated while the immutable borrow is in scope.

    // Iterating over a vector with for loops
    let v3 = vec![100, 32, 57];
    for i in &v3 {
        println!("{i}");
    }

    // Iterating over a mutable vector and mutating elements.
    let mut v4 = vec![100, 32, 57];
    for i in &mut v4 {
        // * is a dereference operator, more in chapter 15.
        *i += 50;
    }

    // NOTE: vectors themselves cannot be mutaded during a for loop, only individual elements.

    // Making vectors with mixed types can be done by defining an enum first.
    // All vector items will be of the enum type `SpreadsheetCell`.
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in &row {
        println!("{:?}", i);
    }
}

fn strings() {
    // The only core string type is the string slice `str`, usually seen as reference `&str`.
    // This includes string literals using `let var = "string goes here"`, which are compiled into the binary.

    // `String` types are from the standard library, not the Rust core.
    // They are growable, mutable, owned, and also UTF-8 encoded same as string slice `&str` types.

    // `String` types are wrappers around a vector of bytes with some extra rules.
    // This means they will usually have the same or similar features as vectors.

    // Building a string starting empty, no type is required, as it's defined with the type itself.
    let mut s1 = String::new();
    println!("`s1` is an empty string: '{}'", s1);

    // Building a string from another set of data. The other set of data must have the `Display` trait.
    let data = "bar";
    let s2 = data.to_string();
    // This also works as:
    // let s2 = "bar".to_string(); // directly from string literal
    // let s2 = String::from("bar"); // frm string literal, using `String::from`
    // `String::from` and `to_string` do the same thing.

    // Different ways to grow a string.
    s1.push_str("foo"); 

    // `push_str(var)` would also work with a string literal: `let s2 = "bar";`
    s1.push_str(&s2); // `push_str()` takes a `&str`, so it doesn't take ownership of what's given to it.
    s1.push('!'); // `push()` takes only a single character/byte, and requires single quotes.
    println!("This is a modified string: '{}'", s1);

    // The + operator also works for `String` types.
    let s3 = String::from(" is a cool word.");
    // NOTE: The `+` operator calls the `add` method, which will take ownership of the first argument
    //       and a reference to the second, after this point, `s1` is no longer in scope.
    let s4 = s1 + &s3;
    println!("This is a string joined with `+`: '{}'.", s4);

    // If more `String` types need to be joined, `+` gets unwieldy, and the `format!` macro works better.
    let s5 = String::from("tic");
    let s6 = String::from("tac");
    let s7 = String::from("toe");

    // `format!` uses references, and works like `println!` so it's easier to deal with.
    let s8 = format!("{s5}-{s6}-{s7}");
    println!("This is a string made with `format!`: '{}'.", s8);

    // NOTE: Not all UTF-8 encoded "characters" are 1 byte, like the latin alphabet is.
    //       Due to this, `String` indexing with `[Int]` is not allowed. Additionally
    //       while slicing with `String[0..4]` is allowed, if the slice happens on a 
    //       boundary that cannot be pretty printed, it will compile, but crash at runtime.

    // To loop through a string, you can use `.chars()` or `.bytes()`.
    for c in "Зд".chars() {
        println!("{c}");
    }
    // Output here will be each human readable character: З and д
    // This will not work with diacritics for example in the Devanagari script.

    for c in "Зд".bytes() {
        println!("{c}");
    }
    // Output here will be the UTF-8 byte-code value of each byte. Not human-readable.
    // Output: 208, 151, 208, 180

    // Contains and replace methods.
    let s9 = String::from("Oompa Loompa");
    println!("`s9`: {s9}");
    println!("Does s9 contain 'o'? {}", s9.contains("o"));

    println!("Replacing 'o' with 'i': {}", s9.replace("o", "i"));
}

fn hashmap() {
    // `HashMap` is in the standard library, but not built into the core.
    use std::collections::HashMap;

    // Initializing an empty hashmap. Types don't need to be defined at initialization.
    // HashMaps honor `Copy` traits, so i32 for example would be copied, but `String` var
    //       would go out of scope, now being owned by the HashMap.
    // References are a special case, wait until lifetimes in Chapter 10.
    let mut scores = HashMap::new();

    // Inserting values into the hashmap with `.insert()`.
    // Once one key:value pair is stored, all future k:v pairs must match those data types.
    // k can have a different type from v, but all k need to be homogenous, same for all v.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Retrieving a key using `.get()`.
    let team_name = String::from("Blue");
    // `.get()` returns Option<&V>
    // `.copied()` returns an Option<i32> instead of Option<&i32>
    // `.unrwap_or(0)` sets score to 0 if scores doesn't have an entry for the key.
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Using get: {team_name}: {score}");

    // Iterating using a loop.
    for (key, value) in &scores {
        println!("Using a for loop: {key}: {value}.");
    }

    // Handling values for pre-existing keys.
    // Overwriting old value:
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // Blue gets it's value overwritten here.

    println!("{:?}", scores);

    // Adding only if not pre-existing value:
    scores.insert(String::from("Yellow"), 10);

    scores.entry(String::from("Yellow")).or_insert(50); // Yellow already has a vlue, it won't be overwritten.
    scores.entry(String::from("Green")).or_insert(50); // Green doesn't have a value, it will be inserted.

    println!("{:?}", scores);

    // Updating old value:
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() { // Returns an iterator over sub-slices, separated by whitespaces of `text`.
        let count = map.entry(word).or_insert(0); // If no v, 0, else, return `&mut v`.
        *count += 1; // de-reference to modify the value being referenced.
    }
    
    println!("{:?}", map);
}