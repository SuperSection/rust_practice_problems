/* Problem Question 1.
*
* Write a function `is_even` that takes a number as an input
* and returns true if it is even
*/


fn main() {
    let num: i32 = 510;
    let ans: bool = is_even(num);
    println!("{} {} an even number", num, if ans { "is" } else { "is NOT" });
}


// rust follows snake_case_notation

fn is_even(num: i32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
