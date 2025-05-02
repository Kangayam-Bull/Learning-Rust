    //defining a struct to hold on to multitude of the values,
    struct User {
        name:String,
        email:String,
        age:u32,
        human:bool
    }
    struct Point(i32,i32, i32);
    struct Colour(i32,i32, i32);

fn main() {
    
    let black = Colour(0,0,0);
    let dark_black = black;//The value of the black will get moved
    let origin = Point(dark_black.0,dark_black.0,dark_black.0);
    println!("{}, {}", origin.0, dark_black.2);


    let mut bala = User {
        name: String::from("Balamurugan K"),
        email: String::from("bmurugan.benz@gmail.com"),
        age: 23,
        human: true
    };

    bala.human = false;

    println!("My name is {}", bala.name);
    println!("is bala a human?\n{}", if (bala.human){
        "Yes!"
    }else{
        "No!!!"
    });

    let witcher = build_user_details(
        String::from("Geralt"),
        String::from("geralt@wither.com"),
        50,
        false);

        println!("Name of the User is {}", witcher.name);
}

fn build_user_details(name:String, email:String, age:u32, human:bool) -> User {

    // User{
    //     name:name,
    //     email:email,
    //     age:age,
    //     human:human
    // };
    User{
        name,
        email,
        age:age,
        human:human
    }
    
}
