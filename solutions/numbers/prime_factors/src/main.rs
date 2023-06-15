use std::io::stdin;

fn main() {
    let mut str_in = String::new();
    stdin().read_line(&mut str_in).expect("input has not been read");
    let u_input: i32 = str_in.trim().parse().expect(" I want a number");
    let mut n = u_input;
    let mut primes: Vec<i32> = Vec::new();
    for x in 2..(u_input-1){
        while n%x == 0{
            primes.push(x);
            n = n/x;
        }
     }
    println!("Printing vec {:?}", primes);
}
