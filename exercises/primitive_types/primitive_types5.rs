// primitive_types5.rs
//
// Destructure the `cat` tuple so that the println will work.
//
// Execute `rustlings hint primitive_types5` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let (name, age) = ("Furry McFurson", 3.5);
    let cat = (name, age);

    println!("{} is {} years old.", cat.0, age);
}
