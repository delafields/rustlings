// This shopping list program isn't compiling!
// Use your knowledge of generics to fix it.

// Execute `rustlings hint generics1` for hints!

fn main() {
    // SOLVED, added borrowed string to Vec
    let mut shopping_list: Vec<&str> = Vec::new();
    shopping_list.push("milk");
}
