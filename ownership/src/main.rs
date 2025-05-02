fn main() {
    let str1 = String::from("Bala is Cool");
    let str2 = GetBack_the_String(str1);

    //println!("{}", str1); //This is not possible because of the ownership is transfered to 
    //the GetBack_the_String(str1); function

    //But whereas the following print will work;

    println!("{}", str2);

    let str3 = String::from("Bala is more cool");
    let str4 = str3.clone();//This is similar to deep copy where the contents in the heap is clonned fully 

    println!("{}", str3);
    println!("{}", str4);

    let num1 = 21;

    let num2 = GetBack_the_Number(num1);
    
    println!("{}", num1);
    println!("{}", num2);
}

fn GetBack_the_String(strvar:String) -> String {

    return strvar;
}

fn GetBack_the_Number(numvar:i32) -> i32 {

    return numvar;
}