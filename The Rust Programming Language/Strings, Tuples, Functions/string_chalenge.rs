//Instructions:
    //-Concatenate a multi string and a simple srting in 2 different ways

pub fn string_chalenge(){
    println!("----------String Challenge-----------");

    
    let multi_string = "Hello 
    Mike, ";
    let single_string = "this is Ana";

    //Concat 1
    let concat1 = multi_string.to_string() + &single_string;
    println!("First concat = {}", concat1);



    //Concat 2
    let mut concat2 = "Hello 
    Mike, ".to_string();
    concat2.push_str(single_string);
    println!("Second concat = {}", concat2);
    assert_eq!(concat1, concat2);
}