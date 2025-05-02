#[derive(Debug)]
struct Rect {
    length:u32,
    breadth:u32
}
fn main() {
    let rect1 = Rect {
        length:49,
        breadth:32
    };

    println!("The value of rect1 = {:#?}", rect1);//The normal {} format spec cannot be used for the struct
    //The Struct data structure does not have Display trait implemented. And we can print the same by using 
    //Debug trait implemented
    println!(
        "The Area of the rectangle = {}",
        area_of_rectangle(&rect1)
    );
}

fn area_of_rectangle(rect1:&Rect) -> u32 {
    rect1.length*rect1.breadth
}
/*
//Second better way of doing this with the tuple, i.e combining the related vars into a single data structure
fn main() {

    let rect:(u32,u32) = (30, 50);

    println!(
        "The area of the rectangle is {}",
        area_of_rectangle(rect)
    );
}

fn area_of_rectangle(dimensions:(u32,u32)) -> u32 {
    dimensions.0*dimensions.1
}
*/
/*
//First try with variables
fn main() {
    let lenght:u32 = 50;
    let breadth:u32 = 49;

    println!(
        "The value of the rectangle = {}", 
        area_of_rectangle(lenght, breadth)
    );
}

fn area_of_rectangle(length:u32, breadth:u32) -> u32 {
    length*breadth
} 
*/
