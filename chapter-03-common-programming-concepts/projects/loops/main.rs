// fn main() {
//     loop {
//         println!("again!");
//         println!("new loop")
//     }
// }

fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;
        println!("counter is {}", counter);

        if counter == 20 {
            break counter * 2;
        }
    };

    println!("The result is {result}")
}
