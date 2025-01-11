// fn main() {
//     // let s = "hello"; // size is known at compile time -> stored on the stack
//     // let mut s = String::from("hello"); //size is unknown at compile time -> stored on the heap

//     // s.push_str(", world!");

//     // println!("{}", s);

//     // let x = 5;
//     // let y = x;

//     let s1 = String::from("hello");
//     let s2 = s1;

//     println!("{s1}, {s2}");
// }

// fn main() {
//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);

//     println!("The length of '{s1}' is {len}.");
// }


// fn calculate_length(s: &String) -> usize {
//     s.len()
// }

fn main() {
    // let string = first_word("hellojj world");
    let (first, second) = second_word("hellojjworld");

    println!("{}, {}", first, second);
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    // let new = bytes.iter().enumerate();
    let new = bytes.iter();

    println!("{:?}", new);

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }    
    }

    s.len()
}

fn second_word(s: &str) -> (usize, usize) {
    let bytes = s.as_bytes();

    let mut first = 0;
    let mut second = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            first = i;
            break;
        }    
    }

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            second = i;
            break;
        }    
    }

    (first, second)
}
