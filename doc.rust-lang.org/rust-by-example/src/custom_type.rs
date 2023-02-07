#[allow(dead_code)]
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

#[allow(dead_code)]
struct Unit;

#[allow(dead_code)]
struct Pair(i32,i32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point
}
impl Rectangle {
    fn square(point: Point, distance: f32) -> Rectangle {
        let Point { x: left, y: top } = point;
        Rectangle {
            top_left: point,
            bottom_right: Point {
                x: distance + left,
                y: top - distance,
            }
        }
    }

    fn rect_area(&self) -> f32 {
        let Point {x: left, y: top} = self.top_left;
        let Point {x: right, y: bottom} = self.bottom_right;

        (right - left) * (top - bottom)
    }
}

pub fn execute_custom_type() {
    let rectangle = Rectangle{
        top_left: Point { x: 10.0, y: 3.0 },
        bottom_right: Point { x: 2.0, y: 8.0}
    };
    println!("{}", rectangle.rect_area());

    let rectangle2 = Rectangle::square(Point { x: 20.0, y: 50.0 }, 10.0);
    println!("{:?}", rectangle2);
}