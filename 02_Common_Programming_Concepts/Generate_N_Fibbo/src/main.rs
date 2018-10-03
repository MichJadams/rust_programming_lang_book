use std::io; 
fn main() {
    println!("input the length of fibbo you want to get to"); 
    let mut fibbo_index = String::new(); 
    io::stdin().read_line(&mut fibbo_index)
        .expect("couldn't read line"); 
    let fibbo_index: u32 = fibbo_index.trim().parse()
        .expect("enter a number"); 
    let mut fib_number:u32 = 0;
    let mut fib_minus_one: u32 = 0; 
    let mut fib_minus_two: u32 = 1;
    
    for i in 0..fibbo_index{
        let mut old_fib = fib_number.clone(); 
        fib_number = fib_minus_one + fib_minus_two; 
        fib_minus_two = fib_minus_one; 
        fib_minus_one = fib_number; 
        println!("this is fib number now: {}", fib_number);
    }
    println!("this is the fib number {}",fib_number); 
}
