// functions2.rs
// Make me compile! Execute `rustlings hint functions2` for hints :)

// I AM DONE 2021-05-04 by stphnsmpsn

fn main() {
    call_me(3);
}

fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
