// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    // SOLVED added if let
    if let Some(x) = option {
    // for x in option {
        res += x;
    }
    println!("{}", res);
}
