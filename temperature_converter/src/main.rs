use std::io;


fn f_to_c(temperature: isize) -> isize {
// F to C: C = (F-32)*(5/9)
    (temperature - 32) * 5/9
}

fn c_to_f(temperature: isize) -> isize {
// C to F: F = C*(9/5) + 32
    (temperature * 9/5) + 32
}

fn main() {
    let temp = loop {
        println!("What temperature are we converting? (integers only)");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read line");

        let temp: isize = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        break temp;
    };

    loop {
        println!("Is this temperature in Celsius or Fahrenheit? input C or F");

        let mut convert_type = String::new();

        io::stdin()
            .read_line(&mut convert_type)
            .expect("Temperature type not accepted.");
    
        if convert_type.trim() == "C" {
            println!("{} converted from Celsius to Fahrenheit: {}", temp, c_to_f(temp));
            break;
        } else if convert_type.trim() == "F" {
            println!("{} converted from Fahrenheit to Celsius: {}", temp, f_to_c(temp));
            break;
        } else {
            println!("Please only input C or F... try again!");
            continue;
        }
    }
}
