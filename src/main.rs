use std::env;

fn main() {
    let mut count = 0;
    
    println!("When I grow up, I wanna be an AUR helper!");
    println!("Meanwhile, here are your arguments <3");
    for argument in env::args() {
        count += 1;
        println!("{}· {}", count, argument);
    }
}
