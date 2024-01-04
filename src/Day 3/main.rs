use std::io;
fn main() {
    let mut array=[3,2,5,1,4];
    array.sort();
    let mut user_input=String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read line");

    let number:usize=user_input.trim().parse().expect("Please enter a valid number ");
    println!("{}",number);

    if number<array.len(){

        println!("{}th smallest element of array is {}",number,array[number-1]);
    }


}