//Instructions:
    //- Create a function to calc the area of a retangule;
    //- The function should be called rect_area;
    //- Use Tuples and structs
    //- Give two retangule points coordinate point1(x,y)& point2(x,y)
    //- height = point2.y-point1.y;
    //- width = point2.x-point1.x

//Struct Pair
struct Pair(i32, f32);

//Points
struct Point {
    x: f32,
    y: f32
}
//Rectangle
struct Rectangle {
    p1: Point,
    p2: Point
}

fn rect_area(_rectangle: &Rectangle){
    let height = _rectangle.p2.y - _rectangle.p1.y;
    println!("Height of the rectangle = {}", height);
    let width = _rectangle.p2.x - _rectangle.p1.x;
    println!("Width of the rectangle = {}", width);

    println!("Area of the rectangle = {}", height*width);

}
pub fn tuples_and_struct_challenge(){
    println!("-----------Tuples & Structs Challenge-----------");

    println!("\n\n Problem:\n\n");

    //Inititate a point
    let point1: Point = Point{x:3.0, y:6.0};
    let point2: Point = Point{x:12.0, y:17.0};
    println!("Point 1 coordinates: x = {} and y = {}", point1.x, point1.y);
    println!("Point 2 coordinates: x = {} and y = {}", point2.x, point2.y);

    //Initiate rectangle
    let rect1 = Rectangle{
        p1: point1, 
        p2: point2
    };

    //Passing retangle by referenace so we can destruct it inside the function
    rect_area(&rect1);

    //Tuples
    let pair = Pair(1, 0.1);
    println!("Pair value 1: {}
    Pair value 2: {}", pair.0, pair.1);

}