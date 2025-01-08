fn main() {
    // let s = "hello"; // size is known at compile time -> stored on the stack
    let mut s = String::from("hello"); //size is unknown at compile time -> stored on the heap

    s.push_str(", world!");

    println!("{}", s);
}
