#[derive(Debug)]
struct Rectangle {
    length:i32,
    breadth:i32,
}
impl Rectangle {

    fn area(&self) -> i32 {
        return self.length*self.breadth;
    }

}
fn main() {

    let rect = Rectangle {
        length:32,
        breadth:63,
    };

    println!("The Area of the rect = {}", rect.area());
    
}
