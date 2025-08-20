/* Problem Question 3.
*
* Write a function `get_string_length` that takes a string
* as an input and returns its length
*/

fn get_string_length(s: &str) -> usize {
    s.chars().count()
}

fn main() {
    let my_string = String::from("Let's Learn Rust!");
    let length = get_string_length(&my_string);
    println!("The number of characters in this string is: {}", length);
}
