// fn notmain() {
//     let l = another_function(5, 65535);
// }
//  fn another_function(x :u32, y: u16) {

// println!("The value of x = {}",x);
//  }

 fn main() {
    let x = 5;

    let y: i32= {
          x + 1 //Expression in a function  
    };

    println!("The value of y is: {}", y);
}
