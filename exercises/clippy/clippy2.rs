// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

// I AM DONE 2021-05-04 by stphnsmpsn

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
