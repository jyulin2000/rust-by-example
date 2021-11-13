use std::fmt;

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}


impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Top left: {}, Bottom right: {}", self.top_left, self.bottom_right)
    }
}

// Calculate the area of a given Rectangle
fn rect_area (rect: &Rectangle) -> f32 {
    let Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: Point { x: right_edge, y: bottom_edge}
    } = rect;

    (top_edge - bottom_edge) * (right_edge - left_edge)
}
/*
fn rect_area (rect: &Rectangle) -> f32 {
    let width: f32 = rect.top_left.y - rect.bottom_right.y;
    let height: f32 = rect.bottom_right.x - rect.top_left.x;
    width * height
}
*/
fn generate_square (bottom_left: &Point, height: f32) -> Rectangle {
    let top_left = Point { x: bottom_left.x, y: bottom_left.y + height };
    let bottom_right = Point { x: bottom_left.x + height, y: bottom_left.y };
    Rectangle { top_left, bottom_right }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);


    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    //let bottom_right = Point { x: 5.2, ..point };
    let bottom_right = Point { x: 5.3, y: 5.4 };
    
    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };
    
    println!("Rectangle: {}", _rectangle);
    println!("Area: {}", rect_area(&_rectangle));
    
    let height = 10f32;
    let bottom_left = _rectangle.bottom_right;
    let square = generate_square(&bottom_left, height);
    println!("Square with bottom left at {} and height of {}: {}", &bottom_left, height, square);
    println!("Area of square: {}", rect_area(&square));
    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}

