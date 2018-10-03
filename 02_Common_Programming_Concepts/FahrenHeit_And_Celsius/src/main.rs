use std::io;
fn main() {
    println!("Enter Your number in Celsius"); 
    let mut temperature = String::new(); 

    io::stdin().read_line(&mut temperature)
        .expect("failed to readline"); 
    let temperature : f32 =  temperature.trim().parse()
        .expect("failed to readline");
    let temperature_f : f32 = temperature*1.8+32.0;
    println!("this is your temperature in fahreinheit {}", temperature_f); 
}
