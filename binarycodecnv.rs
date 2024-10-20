use std::io::{self, Write};

fn main(){
    let mut user_query = String::new();
    println!("This program helps convert entered *numbers to binary");
    print!("Number to Convert: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut user_query).expect("Faild to read input");
    let user_number : i32  = user_query.trim().parse().expect("You need to enter a valid int/number");
    println!("BinaryFormat: -> {:b}", user_number); //Display the int in binary
}
