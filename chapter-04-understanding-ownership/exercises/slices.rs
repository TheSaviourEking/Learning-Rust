fn main() {
    let s = String::from("hello world");

    // let hello: &str = &s[0..5];
    // let world: &str = &s[6..];

    // let s2: &String = &s;

    // println!("{}", s2);
    

    let word = first_word(&s);
    // let word = second_word(& s);

    println!("{}, - ,{}", word, &word);

    println!(
        "&String={} &str={}",
        std::mem::size_of::<&String>(),
        std::mem::size_of::<&str>(),
    );
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let my_string = String::from("hello world");

    println!("{}", my_string);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }

    &s[..]
}
