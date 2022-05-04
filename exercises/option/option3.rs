// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // SOLVED added ref
        // this informs the compiler we don't want a move to happen
        // instead we just borrow
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
