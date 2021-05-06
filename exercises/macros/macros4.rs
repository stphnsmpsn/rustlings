// macros4.rs
// Make me compile! Execute `rustlings hint macros4` for hints :)

// rustfmt ftw... the key here was knowing that you need something between each "marco arm"
// I AM DONE 2021-05-06 by stphnsmpsn

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
