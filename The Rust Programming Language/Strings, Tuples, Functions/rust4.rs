#[derive(Debug)]//When there are printing errors of complex variabels
struct GenericStruct<T>(T);//A struct with an optional

struct Point {
    x:f32,
    y:f32
}

pub fn rust4(){
    
    //Slicing
    println!("-----Slicing-----");
    let  vec = vec![1,2,3];
    let int_slice = &vec[..];//Create slice from vector: Copies the vector values but it is 
                             //an object with different properties
    let smaller_slice = &vec[0..2];//Sliced form of vec  = [1,2]
    let str_slice: &[&str] = &["one", "two", "three"];//Create a new slice of type str

    println!("vec = {:?}", vec);//prints "vec=[1, 2, 3]"
    println!("int_slice = {:?}", int_slice);//prints a copy o vec "int_slice = [1, 2, 3]"
    println!("smaller_slice = {:?}", smaller_slice);//prints a sliced copy o vec "smaller_slice = [1, 2]"
    println!("str_slice = {:?}", str_slice);//prints "str_slice=["one", "two", "three"]"

    assert_eq!(str_slice.len(), 3);//Prints nothing if it is correct
    //assert_eq!(str_slice.len(), 2);//Prints error
    assert!(!str_slice.is_empty());

    //Strings
    println!("-----Strings-----");

    //concatenation 1st way
    let mut s = "Hello".to_string();
    println!("s={:?}", s);//Prints "s="Hello""
    s.push_str(", world.");//push_str() is the way to concatenate strings in Rust
    println!("s={:?}", s);//Prints "s="Hello, world.""
    

    //concatenation 2nd way
    let hello ="Hello ".to_string();
    let world = "world!".to_string();
    let hello_world = hello +&world;//Without the .to_string() it would not let us join this two
    println!("{}", hello_world);

    //multiple lines
    let multi_string = "hey
    Brett";
    println!("{}", multi_string); //Recognizes and prints two separate lines, just like it is written
                                  //in the IDE. prints "hey
                                  //                    Brett"
    //assert_eq!("heyBrett", multi_string);//Fails
    assert_eq!("hey
    Brett", multi_string);//Passes

    //Tuples
    println!("-----Tuples-----");
    let tuple = ("Member1", true, "Member3", 2);
    println!("First Element of tuple: {}", tuple.0);//prints: "First Element of tuple: Member1"
    println!("Second Element of tuple: {}", tuple.1);//prints: "Second Element of tuple: true"
    println!("Third Element of tuple: {}", tuple.2);//prints: "Third Element of tuple: Member3"
    println!("Fourth Element of tuple: {}", tuple.3);//prints: "Fourth Element of tuple: 2"

    //Generics
    let generic_var: GenericStruct<char> = GenericStruct('a');//Create struct of type GenericStruct
    println!("first generic {:?}", generic_var);//prints "first generic GenericStruct('a')"
    let another_generic: GenericStruct<bool> =GenericStruct(true);
    println!("second generic {:?}", another_generic);//prints "first generic GenericStruct(true)"

    let point: Point = Point{x:0.3, y:0.4};
    println!("point coordinates: ({}, {})", point.x, point.y);
}