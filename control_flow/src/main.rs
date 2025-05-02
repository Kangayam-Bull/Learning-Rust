use std::io;

use io::stdin;

fn _conditional_main() {
    let x = 3;
    if x < 5 {
        println!("x is less than 5");
    } else if x > 5 {
        println!("x is greater than 5");
    } else {
        println!("x is equal to 5");
    }
}

fn _infy_main () {
    loop {
        println!("again!");
    }
}

fn _loopmain() {
    let mut counter = 0;

    let result = 12;
    loop {
        counter += 1;
        println!("The Counter is 10");
        if counter == 10 {
            
            break counter;
            //break counter * 2;//This statement after the break key word is evaluated and returned to the loop's result.
            //counter*12// there is no direct return as we do in the expression, and break comment should be used
        }
    };

    println!("The result is {}", result);
}


fn _while_main () {
    let mut countdown :i32 = 3;

    while countdown != 0 {
        println!("{}", countdown);
        countdown -= 1; //decrementing
    } 
    println!("LIFTOFF!!")
}

fn _formain() {
    let a:[i32;3] = [3;3];

    for element in a.iter() {
        println!("The value is {}", element);
    }
}

fn _formain2 () {
    let mut countdown :i32 = 10;
    for countdown in (1..4).rev() {
        println!("{}", countdown);
    }
}
fn _f_to_c_main () {
    let mut input_string = String::new();

    io::stdin()
    .read_line(&mut input_string)
    .expect("Please enter a Fahrenheit value");

    let fahrenheit_input :f64 = input_string.trim().parse().expect("Enter a valid number");

    println!("The equivalent celcius scale is {}",(fahrenheit_input-32.0)*(5.0/9.0));
}

fn main() {
    let mut input_string = String::new();
    println!("enter any index number to calculate the fib value of the same");
    io::stdin()
    .read_line(&mut input_string)
    .expect("Enter a valid number");

    let number:u32 = input_string.trim().parse().expect("Enter a valid positive whole number");

    // for element in (0..fibno(number)+1) {
    //     for count in (0..element+1) {
    //         print!("{}","*");
    //     }
    //     println!("");
    // }
    println!("{}", fibnorecursion(number));

    // let number: u64 = 4294967295;
    // print!("{}", number.max(12586269025));

    println!("Factorial of {} is {}", 5, factorial(1500));
    example_loop();
    example_for();
}   

fn factorial(input: i32) ->i32 {

    if input == 1{
        return 1;
    }
    return input+factorial(input-1);

}

fn fibnorecursion(number:u32) -> u32 {
    if (number<=1){
        return number;
    }
    return fibnorecursion(number-1)+fibnorecursion(number-2);

}

fn fibno(number:u32) -> u32 {

    let mut fib1 = 0;
    let mut fib2: u32 = 1;
    if(number == 0)
    {
        fib2 = 0;
    }
    else if (number == 1)
    {
        fib2 = 1;
    }
    else 
    {
        for element in (2..number) {
            let sum = fib1+fib2;
            fib1 = fib2; 
            fib2 = sum;
        }
    }

    return fib2;

}

fn example_loop() {
    let mut number = 0;

    let mut return_number = loop{
        number += 1;
        if (number >10)
        {
            break number;
        }
    };

    loop {

        return_number += 1;

        break;
    }

    println!("The return_number = {}", return_number);
}

fn example_for() {
    let temp_array:[i32;5] = [1,2,3,4,5];
    for element in temp_array.iter() {
        println!("{}", element);
    }
}