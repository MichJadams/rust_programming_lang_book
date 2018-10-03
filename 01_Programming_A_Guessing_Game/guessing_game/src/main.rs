extern crate rand;
use std::io;   
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!"); 
    
        println!("Please input your guess:");
        let random_number = rand::thread_rng().gen_range(1,101);
    loop{
        let mut guess = String::new(); 
        io::stdin().read_line(&mut guess)
            .expect("failed to read line");

        println!("you guessed {}:",guess);
        let guess : u32 = match guess.trim().parse(){
            Ok(num)=> num, 
            Err(_) => continue,
        };        
        match guess.cmp(&random_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal =>{
                println!("You Win!");
                break;
            }
        }
    } 
}
