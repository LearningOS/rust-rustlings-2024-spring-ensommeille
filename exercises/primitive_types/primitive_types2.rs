// primitive_types2.rs
//
// Fill in the rest of the line that has code missing! No hints, there's no
// tricks, just get used to typing these :)
//
// Execute `rustlings hint primitive_types2` or use the `hint` watch subcommand
// for a hint.

use std::mem;

fn main() {
    // Characters (`char`)

    // Note the _single_ quotes, these are different from the double quotes
    // you've been seeing around.
    let my_first_initial = 'C';
    if my_first_initial.is_alphabetic() {
        println!("Alphabetical!");
    } else if my_first_initial.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }

    let your_character = '\u{21252}';// Finish this line like the example! What's your favorite character?
    // Try a letter, try a number, try a special character, try a character
    // from a different language than your own, try an emoji!
    if your_character.is_alphabetic() {
        println!("Alphabetical!");
    } else if your_character.is_numeric() {
        println!("Numerical!");
    } else {
        println!("Neither alphabetic nor numeric!");
    }
    println!("{}", your_character);
    println!("{}", mem::size_of_val(&your_character));
    let a = String::from("\u{22234}");
    let b = String::from("dasf");

    // 获取 a 和 b 内部实际存储的字节数
    let bytes_a = a.as_bytes().len();
    let bytes_b = b.as_bytes().len();

    let len_a = a.len();
    let len_b = b.len();

    println!("字符串 a 内部实际存储的字节数为: {}", bytes_a);
    println!("字符串 b 内部实际存储的字节数为: {}", bytes_b);

    println!("字符串 a 的长度为: {}", len_a);
    println!("字符串 b 的长度为: {}", len_b);

    println!("{}", a);
    let c = '\u{100230}';
    let size = std::mem::size_of_val(&c);

    println!("字符 {} 的 UTF-32 编码单元数为: {}", c, size);
}
