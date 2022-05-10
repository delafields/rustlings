// macros2.rs
// Make me compile! Execute `rustlings hint macros2` for hints :)

// SOLVED macros need to be defined before they're used
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}

// macro_rules! my_macro {
//     () => {
//         println!("Check out my macro!");
//     };
// }
