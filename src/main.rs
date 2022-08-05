use std::io;

fn main() {
    println!("Hello, VIN demo!");

    println!("VIN: ");

    let mut vin = String::new();

    io::stdin()
        .read_line(&mut vin)
        .expect("Failed to read line");
    
    println!("VIN is {vin}");

    assert!(vin::check_validity(&vin).is_ok());
}
