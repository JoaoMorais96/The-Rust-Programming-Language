struct Student {
    age: i32,
    mark: i32
}
#[derive(Debug)]
enum Languages {
    Java,
    Scala,
    Rust, 
    C,
    Swift
}

fn check_division(dividend:i32, divisor:i32) -> Option<i32>{
    if divisor == 0 {
        None
    }else{
        Some(dividend/divisor)
    }
}

fn try_division(dividend:i32, divisor:i32){
    match check_division(dividend, divisor){
        None => println!("{}/{} failed!", dividend, divisor),
        Some(quotient) => {
            println!("{}/{} = {}!", dividend, divisor, quotient);
        }
    }
}

pub fn main3(){

    //Structures usage
    let student:Student = Student {age: 23, mark:17};
    //let student_dos = Student {age: 13, mark:10};//You can declare without the :Student
    println!("Student info: age {} and mark {}", student.age, student.mark);// prints "Student info: age 23 and mark 17" 

    let Student{age:my_age, mark:my_mark} = student;
    println!("Info gathered: {} and {}", my_age, my_mark);// prints "Info gathered: 23 and 17" 

    //Enums
    let first_language = Languages::Java;
    println!("{:?}", first_language);//Prints "Java"

    //Option <T>
    //None ==> Absence of a value or failure of processing
    //Some(v) ==> a value with type "T"

    let number = Some(7);
    let mut letter: Option<i32> = None;

    if let Some(i) = number {
        println!("Matched {:?}", i);//prints "Matched 7"
    } else {
        println!("Didn't match a number.");
    }

    if let Some(i) = letter {
        println!("Matched {:?}", i);
    }else{
        println!("Didn't match a letter.");//prints "Didn't match a letter."
    }

    letter = Some(40);

    if let Some(i) = letter {
        println!("Matched {:?}", i);//prints "Matched 40"
    }else{
        println!("Didn't match a letter.");
    }
    
    //Call Option<T> functions
    try_division(4,3);// Prints "4/3 = 1!"
    try_division(4,0);// Prints "4/0 failed!"

    //Arrays
    let my_array = [5, 7, 8];//Arrays are immutable by default
    println!("Here are the elements of my array: {:?}", my_array);//Prints "Here are the elements of my array: [5, 7, 8]"

    let mut my_mutable_array = [1, 2, 3];
    println!("Here are the elements of my_mutable_array: {:?}", my_mutable_array); //Prints "Here are the elements 
                                                                                    //of my_mutable_array: [5, 7, 8]"
    let mut index = 0;
    println!("Element one = {}", my_array[0]);//Prints "Element one = 1"
    for i in my_array.iter() {
        println!("@index = {}, the value is {}",index, i); //@index = 0, the value is 5
                                                            //@index = 1, the value is 7
        index = index + 1;                                  //@index = 2, the value is 8
    }

    let array_two = [4; 5]; //Array of 5 instance sof the number 4
    println!("array_two = {:?}", array_two); //Prints "[4, 4, 4, 4, 4]"
    for i in 0..array_two.len(){
        println!("@index = {}, the value is = {}",i, array_two[i])//@index = 0, the value is = 4
    }                                                             //...
                                                                  //@index = 4, the value is = 4

    //Vectors
    let mut my_vector = vec![4,7,8,9];
    let another_vector = vec![0;10];
    println!("The 4th element of my first vector is {}", my_vector[3]);//Prints "The 4th element of my first vector is 9"
    my_vector.push(0);//Places "0" in the last house of the arry;
    println!("The last element added to my first vector is {}", my_vector[5]);//Prints "The last element added 
                                                                              //to my first vector is 0"
    my_vector.remove(5);//Removes the number at index 5, in this case "0"
    for i in my_vector{ //no need for .inter() in vectors
        println!("element: {}", i);//Prints: element: 1 ... element: 5
    }
}