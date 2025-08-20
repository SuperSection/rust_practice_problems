/* Problem Question 4.
*
* Write an example of struct in rust
*
* -- Structs let you structure data together
* -- As you see in C/C++, also similar to 'Object' in JavaScript
*/

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("supersection"),
        email: String::from("soumosarkar.official@gmail.com"),
        sign_in_count: 5,
    };

    println!(
        "User1 username: {}\nStatus: {}\nEmail: {}\nHow many times signed in? {}",
        user1.username,
        if user1.active { "active" } else { "inactive" },
        user1.email,
        user1.sign_in_count
    );
}
