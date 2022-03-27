use std;
fn function_1(){
    let x = 3+5;
    println!("3 + 5 = {}", x);
}
fn function_2(name:String){
    println!("Hello {}",name);
}
fn sum_of_two_number(x:i32, y:i32){
    println!("{} + {} = {}", x,y,x*y);
}

pub fn functions_and_closures(){
    println!("----------Functions & Closures---------");
    //Functions
    function_1();
    function_2("John Doe".to_string());
    sum_of_two_number(4, 9);

    //Closures
    let var_minus_1 =  |x:i32| x-1;
    println!("{}", var_minus_1(3));//prints "2"

    let plus_two = |x| {
        let mut result:i32 = x;
        result +=1;
        result +=1;
        result
    };
    println!("{}", plus_two(4));//prints "6"
}