use std::io::{self, Write}; //We need to flush the screen to display prompt

fn main(){
    //initialize our variable to store user input
    let mut user_query = String::new(); 
    println!("This program helps convert entered *numbers to binary");
    print!("Number to Convert: ");
    //flush screen to allow the prompt above to be displayed
    io::stdout().flush().unwrap();
    //converting user input to int or return an exception if the input is not valid i.e if user did not enter an int
    io::stdin().read_line(&mut user_query).expect("Failed to read input");
    let user_number : i32  = user_query.trim().parse().expect("You need to enter a valid int/number");
    println!("BinaryFormat: -> {:b}", user_number); //Display the int in binary
}
