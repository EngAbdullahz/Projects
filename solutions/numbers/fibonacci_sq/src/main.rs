use std::io::{stdout, stdin, Write};


fn main(){
    println!("Put a number to calculate its fibonacci sequence");
    stdout().flush().expect("Failed to flush stdout");
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("failed to read input");
    let no_dig: usize = buf.trim().parse().unwrap();
    match no_dig {
       2..=92 => println!("You have chosen {}", no_dig),
       _ => panic!("Plase chose from 2 to 92")
    }
    // Generating fibonacci sequence
    let mut fib_vec :Vec<i64>= Vec::new();
    fib_vec.push(0);
    fib_vec.push(1);
    for x in 2..no_dig{
        fib_vec.push(fib_vec[x-2] + fib_vec[x-1]);
    }
    println!("Printing vec {:?}", fib_vec);
}
