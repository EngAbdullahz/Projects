use std::{f32::consts::PI, io::{stdin, stdout, Write}, panic};

fn main(){
    println!("How many digits do you want pi to end at?");
    stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("failed to read input");
    let no_dig: usize = buf.trim().parse().unwrap();
    match no_dig {
       0..=22 => println!("You have chosen {}", no_dig),
       _ => panic!("Plase chose from 0 to 22")
    }
    println!("Pi = {:.no_dig$}",PI);
}
