//Instructions:
    //-Create a vector fill it with elements
    //-Loop through the vector and print only the number divisible by 3;
    //-If they are not divisible by 3, add them to another vector and print it

pub fn m4_challenge(){
    let my_vector = vec![1,3, 24, 30, 9, 6, 2, 4 ,27, 10];
    let mut non_div_vector = vec![];
    for i in my_vector{
        if i%3 == 0{
            println!("Element {} is divisible by 3", i);
        }else{
            non_div_vector.push(i);
        }
    }
    println!("Non_div_vector = {:?}", non_div_vector);
}