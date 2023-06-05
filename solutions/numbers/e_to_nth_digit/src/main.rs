use std::{f32::consts::E, io::{stdin, stdout, Write}, panic};

fn main(){
    println!("How many digits do you want 'e' to end at?");
    stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("failed to read input");
    let no_dig: usize = buf.trim().parse().unwrap();
    match no_dig {
       0..=20 => println!("You have chosen {}", no_dig),
       _ => panic!("Plase chose from 0 to 20")
    }
    println!("E = {:.no_dig$}",E);
}
