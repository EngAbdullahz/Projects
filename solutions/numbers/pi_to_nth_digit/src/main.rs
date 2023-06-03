fn main() {
    let mut pi_est:f64 = 1.0;
    for x in 1..10000000000 as i64{
        if x%2 == 0{
            pi_est = pi_est + (1.0/(1.0+2.0*{x as f64}));
        }
        else{
            pi_est = pi_est - (1.0/(1.0+2.0*{x as f64}));
        }
    }
    pi_est = pi_est*4.0;
    println!("Pi={}", pi_est);
}
