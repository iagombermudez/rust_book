// Let’s make a new binary project with Cargo called rectangles that will take the width and
// height of a rectangle specified in pixels and calculate the area of the rectangle.

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn fits(&self, rectangle: &Rectangle) -> bool {
        rectangle.height < self.height && rectangle.width < self.width
    }
}

fn main() {
    let width = 10;
    let height = 20;

    let rectangle = Rectangle { width, height };
    let area_rectangle = rectangle.area();
    println!("{area_rectangle}px");

    let rectangle2 = Rectangle {
        width: 5,
        height: 10,
    };

    let fits = rectangle.fits(&rectangle2);
    println!("{fits}");
}
