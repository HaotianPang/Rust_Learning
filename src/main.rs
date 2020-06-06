use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let secret_number=rand::thread_rng().gen_range(1,101);
    println!("the secret number is:{}",secret_number);
    println!("let's play a guessing game!");
    println!("please input your number here!");
    let mut guess=String::new();
    io::stdin()
    .read_line(&mut guess)
    .expect("faild to get an input");
    println!("your guess is {}",guess);
    let guess:u32= match guess.trim().parse(){
        Ok(num)=>num,
        Err(_)=>0,
    };

    match guess.cmp(&secret_number){
        Ordering::Greater=>println!("Too big"),
        Ordering::Equal=>println!("They are equal"),
        Ordering::Less=>println!("Too small"),
    }
}