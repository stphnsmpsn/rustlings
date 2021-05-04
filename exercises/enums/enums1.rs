// enums1.rs
// Make me compile! Execute `rustlings hint enums1` for hints!

// I AM DONE 2021-05-04 by stphnsmpsn

#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
