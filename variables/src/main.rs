const MAX_POINTS: u32 = 10_0000;//Global scope constants
use std::io;

use io::stdin;
fn _notmain() {
    let mut x: i32 = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("The value of MAX_POINTS = {}", MAX_POINTS);

}

fn _notmain2() {
    let  x = 5;

    let mut x = x + 1;

    x = x*2;

    let x = x*3;

    println!("The value of x is: {}", x);
}
fn _spacemain() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("The Spaces = {}", spaces);
}

fn Charmain() {
    let tamil_letter:char = 'அ';//accepts a four bytes unicode chars which includes the ascii

    println!("{}", tamil_letter);
}

fn Tuple_main () {
    let tupleEx = (100, 10.2, 'b');

    //Pattern Mathcing for accessing the items -> this is called destructing
    let (x, y, z) = tupleEx;
    println!("x = {}", x);

    //Indexing for accessing the items
    let x = tupleEx.1;
    print!("x = {}",x);
}

fn main() {
    let a = [1,2,3,4,5,3];
    let months:[&str; 12]  = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    let a: [i32;5] = [3;5];//initializing the array with '3' for five times
    let a: [i32;5] = [1,2,3,4,5];
    let mut index = get_num();
    println!("{}",a[index]);
}

fn get_num() -> usize{
    let mut inputNumber = String::new();
    io::stdin()
    .read_line(&mut inputNumber)
    .expect("Enter a number");

    let inputNumber:usize= inputNumber.trim().parse().expect("Enter a valid number");

    return inputNumber;
}