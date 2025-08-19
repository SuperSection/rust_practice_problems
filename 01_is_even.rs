/* Problem Question 1.
*
* Write a function `is_even` that takes a number as an input
* and returns true if it is even
*/


fn main() {
    let num = 2100234567;
    let ans = is_even(num);
    println!("{} {} an even number", num, if ans { "is" } else { "is NOT" });
}


// rust follows snake_case_notation

fn is_even(num: u32) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
        return false;
    }
}
