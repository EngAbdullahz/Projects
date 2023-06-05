use std::{io::{stdout, stdin, Write}, i8::MAX};


fn main(){
    println!("Put a number to calculate its fibonacci sequence");
    stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("failed to read input");
    let no_dig: usize = buf.trim().parse().unwrap();
    match no_dig {
       0..=20 => println!("You have chosen {}", no_dig),
       _ => panic!("Plase chose from 0 to 20")
    }
    // Generating fibonacci sequence
    let mut fib_vec = Vec::new();
    println!("E = {:.no_dig$}",E);
}
