use std::usize;

fn main() {
    let s = String::from("Rust is awesome");

    let word = first_word(&s);
    println!("{}", word);

    let str = "WTF";//This is an &str
    let chararray:[char;5] =['a', 'b', 'c', 'd', 'e'];
    for i in chararray.iter() {
        println!("{}", i);
    }
    println!("{}", str);

}

fn first_word(s:&String) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }

    return &s[..];
}

fn _first_word_index(s:&String) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }

    return s.len();
}