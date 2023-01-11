use std::io;

fn main() {
    let mut inputa= String::new();  //declares input variable (as a String)
    let mut inputb= String::new();

    println!("Please enter in two numbers.");  //prints a message to the user

    io::stdin().read_line(&mut inputa).expect("failed to read line");   //collects the first number
    let inputa1 : i32 = inputa.trim().parse().unwrap();     //parses first number as an integer

    io::stdin().read_line(&mut inputb).expect("failed to read line");   //collects the second number
    let inputb1 : i32 = inputb.trim().parse().unwrap();     //parses second number as an integer

    math(inputa1,inputb1);      //calls the math function

}

fn math(x:i32, y:i32)->i32{     //declares a function to calculate the sum of the two numbers and see if it's greater then or less then 100

    let z: i32 = x + y;
    if z > 100{
        println!("The sum of your numbers is greater the 100! Congradulations you have the ability to know numbers greater then 10! :D");
        z
    }else {
        println!("Imagine not being able to think of numbers who's sum is greater then 100. :O");
        z
    }

}
