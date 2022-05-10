// macros1.rs
// Make me compile! Execute `rustlings hint macros1` for hints :)

macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    // SOLVED we invoke macros with a !
    my_macro!();
}
