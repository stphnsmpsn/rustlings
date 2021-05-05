// traits1.rs
// Time to implement some traits!
//
// Your task is to implement the trait
// `AppendBar' for the type `String'.
//
// The trait AppendBar has only one function,
// which appends "Bar" to any object
// implementing this trait.

// I AM DONE 2021-05-04 by stphnsmpsn

trait AppendBar {
    fn append_bar(self) -> Self;
    fn append_bar_two(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(mut self) -> String {
        self.push_str("Bar");
        self
    }

    fn append_bar_two(self) -> String {
        format!("{}Bar", self)
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
    let s2 = String::from("Foo");
    let s2 = s2.append_bar_two();
    println!("s: {}", s2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
        assert_eq!(String::from("Foo").append_bar_two(), String::from("FooBar"));

    }

    #[test]
    fn is_BarBar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
        assert_eq!(
            String::from("").append_bar_two().append_bar_two(),
            String::from("BarBar")
        );
    }
}
