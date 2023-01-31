use std::io;

fn fibonacci(n: u128) -> u128 {
    if n == 0 {
        0
    } else if n == 1 || n == 2 {
        1
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}

fn main() {
    let n = loop {
        println!("What number in the fibonacci sequence do you want? (positive integers only)");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u128 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break n;
    };

    let m = n + 1;

    let cached_values: [isize; m]

    for i in 0..m {
        println!("The {}th number in the fibonacci sequence is: {}", i, fibonacci(i))
    }
}
