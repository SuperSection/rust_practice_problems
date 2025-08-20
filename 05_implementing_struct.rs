/* Problem Question 5.
*
* Write an example of implementing struct
* Find area of a rectangle
*
* -- More similar to classes in JavaScript
*/

struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rect {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle ({}x{}) is {}",
        rect.width,
        rect.height,
        rect.area()
    );
}
