// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    // SOLVED this is what's known as "shadow binding"
    // added the below to its own code block {}, giving it its own scope
    {
        let number = 3;
        println!("Number plus two is : {}", number + 2);
    }
}
