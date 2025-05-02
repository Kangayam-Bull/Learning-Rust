fn main() {
    println!("Hello, world!");
    let strings = get_string();
    let len = get_string_length(&strings);
    println!("The Return String is {} and its length is {}", strings, len);
    
    //investigating the combination of references
    let mut new_string = String::from("Dino-chicken");
    let ref1 = &new_string;
    let ref2 = &new_string;
    println!("The values of the refs are as follows\n{}\n{}", ref1, ref2);

    
    {
        let ref4 = &mut new_string;
        println!("ref4 - {}", ref4);
    }
    let ref3 = &mut new_string;
    let ref5 = &new_string;
    println!("The values of the refs are as follows\n{}", ref3);
    println!("{}", ref5);

    //println!("The values of the refs are as follows\n{}\n{}\n{}", ref1, ref2, ref3);
}

fn get_string() -> String{
    let strings = String::from("Balamurugan");

    return strings;//Normal return

    //return &strings//this return will fail as the owner var is going out of scope
}

fn get_string_length(strings: &String) -> usize {
    let len = strings.len();

    return len;
}