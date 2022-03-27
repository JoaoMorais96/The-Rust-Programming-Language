use std::mem; //To see size of variables
use std; //Everything from the standard library

fn foo_function() {
    let y = 5;
    println!("foo function is being called. Result is {}", y);
}

pub fn main1() {
    //data types
    let bool_variable = true;
    println!("My bool variable value is {}", bool_variable);
    let bool_two:bool = true;
    println!("My bool_two variable value is {}", bool_two);
    println!("size of boolean is {} byte", mem::size_of_val(&bool_two));

    let c= 'c';
    println!("size of char is {} byte", mem::size_of_val(&c));

    let number:i64 = 42; //i8, u8, i16, u16, i32, u32, i64, u64,
    println!("c={} and number={}", c, number);
    println!("size of i64 is {} byte", mem::size_of_val(&number));
    let double_number:f32 = 1.0; //f32, f64
    println!("double_number={}", double_number);

    let my_string = "hello string!";
    println!("my_string = {}", my_string);

    //Stack and heap
        //Heap val will be a pointer to the real var stored in the heap. the pointer is always 8bit, regardless of
        //how complex the stored val in the heap really is
    let mut stack = 10;
    let heap = Box::new(10);
    println!("stack={} heap={}", stack, heap);
    println!("size of stack={} ---- size of pointer to heap={} ---- size of heap={}",  mem::size_of_val(&stack),  mem::size_of_val(&heap), mem::size_of_val(&*heap));

    //Scope and shadowing
    let stack = 10;
    foo_function();//Can print the cariables within it, but we can't print them outside of the function

    {//no name function. Not really a function, just a smaller scope within this function
        stack = 21;//shadows the stck variable defined in another socpe
        let scoped_var = 4;
        println!("scoped_var = {} and stack = {}", scoped_var, stack);//within the scope- prints 4 and 21
    }
    
    let scoped_var = 15;//shadows a val already established in another scope
    println!("Redeclared scoped_var = {} and stack = {}", scoped_var, stack);//out of scope- prints 15 and 21

    //operators
    let number = 2;
    let add_number = number + 5;
    let multiply_number  = number * 5;
    let div_number  =  5/number;//2
    let modulo = 5%2;//1
    println!("div_number {}", div_number);

    let mut another_number = 7;
    another_number -= 2;//5

    another_number = 7;
    another_number += 2;//9

    another_number = 7;
    another_number *= 2;//14

    let cubed = i32::pow(number,3);//8

    let cubed_times_Pi = 8.0 * std::f64::consts::PI;

    let mut bool_result = 5<6//true
    bool_result = 5==5//true
    

}