fn my_loop() {
    let mut counter: u8 = 0;

    let result: u8 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}

fn labelled_loop() {
    let mut count: u8 = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining: u8 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_loop() {
    let mut number: u8 = 3;

    while number != 0 {
        println!("{number}...");

        number -= 1;
    }
    println!("LIFTOFF!!!");
}

fn for_loop() {
    let a: [u8; 5] = [10, 20, 30, 40, 50];

    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}...");
    }
    println!("LIFTOFF!!!");
}

fn main() {
    let number: u8 = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3 or 2");
    }


    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    my_loop();

    labelled_loop();

    while_loop();

    for_loop();
}
