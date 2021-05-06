// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// I AM DONE 2021-05-06 by stphnsmpsn

// must define / bring macros into scope before you call them (unlike functions)
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
