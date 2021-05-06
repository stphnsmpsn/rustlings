// macros3.rs
// Make me compile, without taking the macro out of the module!
// Execute `rustlings hint macros3` for hints :)

// I AM DONE 2021-05-06 by stphnsmpsn

#[macro_use] // we would use #[macro_export] if exporting across crates
mod macros {
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
