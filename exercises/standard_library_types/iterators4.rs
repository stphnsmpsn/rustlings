// iterators4.rs

// I AM DONE 2021-05-05 by stphnsmpsn

pub fn factorial(num: u64) -> u64 {
    // folding is useful when you have a collection and you want to produce a single value from it
    // todo: review fold...
    // todo: note: range is only inclusive of the lower bound
    (1..num + 1).fold(1, |prod, x| prod * x)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
