// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

// I AM DONE 2021-05-06 by stphnsmpsn

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
