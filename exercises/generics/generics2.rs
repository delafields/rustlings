// This powerful wrapper provides the ability to store a positive integer value.
// Rewrite it using generics so that it supports wrapping ANY type.

// Execute `rustlings hint generics2` for hints!

// struct Wrapper {
//     value: u32,
// }

// impl Wrapper {
//     pub fn new(value: u32) -> Self {
//         Wrapper { value }
//     }
// }

// SOLVED, reformat Wrapper to accept a generic type vs a concrete type
struct Wrapper<T> {
    value: T
}
// SOLVED, reformat Wrapper to accept a generic type vs a concrete type
impl<T> Wrapper<T> {
    pub fn new(value: T) -> Self {
        Wrapper { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        assert_eq!(Wrapper::new("Foo").value, "Foo");
    }
}
