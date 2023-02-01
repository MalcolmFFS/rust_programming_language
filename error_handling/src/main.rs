use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};

fn main() {
    // Manually causing a panic.
    // panic!("crash and burn");

    // `panic!`-ing for trying to reach an index that doesn't exist.
    // let v = vec![1, 2, 3];
    //
    //v[99];

    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // `.unrwap()` will automatically panic if Err is returned
    // let greeting_file = File::open("hello2.txt").unwrap();

    // `.expect()` will also automatically panic, but you can feed it an error
    // This is usually preferred over `.unrwap()`
    // let greeting_file = File::open("hello2.txt")
    //     .expect("hello2.txt should be included in this project");

    read_username_from_file();
    read_username_from_file_2();
    read_username_from_file_3();
    read_username_from_file_4();

    // The `?` operator cannot be used in standard main(), it has to be in a 
    // function that returns a Result<T, E> or Option<T>
    // let greeting_file = File::open("hello.txt")?;
}

fn read_username_from_file() -> Result<String, io::Error> {
    // This function will return the error to the calling program, instead
    // of handling it internally. This is done by handling the `Err` of a
    // `match` call as follows: `Err(e) => return Err(e),`

    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2() -> Result<String, io::Error> {
    // The same error propagation can be done using `?` as seen here
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_3() -> Result<String, io::Error> {
    // The even shorter version
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

fn read_username_from_file_4() -> Result<String, io::Error> {
    // There is a builtin to handle this, now that learning was done
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    // Example of using ? on Option<T> return values
    text.lines().next()?.chars().last()
}