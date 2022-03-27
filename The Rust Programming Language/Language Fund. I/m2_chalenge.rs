//Instructions:
    //-create two number values;
    //- within a small scope: multiply and store the result in another variable
    //                        print out a variable value
    //- do the same and store it in the same variable and print it
pub fn m2_chalenge_func(){
    let number_one = 3;
    let number_two = 4;
    {
        let multiply_one = number_one * number_two;
        println!("multiply_one = {}", multiply_one)
    }
    let multiply_one = number_one * number_two * 2;
    println!("multiply_one = {}", multiply_one)
}