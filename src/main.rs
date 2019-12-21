use std::io;

fn main() {
    println!("Farenheit Converter!\n");
    println!("Enter farenheit:");

    let far: i32 = loop {
        let mut far = String::new();

        io::stdin().read_line(& mut far)
            .expect("failed to read line");

        let far: i32 = match far.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("OPS! You must enter a number!"); 
                continue
            },
        };

        break far;
    };

    println!("{}f is {}c!", far, farenheit_to_celcius(far));
}

fn farenheit_to_celcius(far: i32) -> i32 {
    (far - 32) * 5/9
}
