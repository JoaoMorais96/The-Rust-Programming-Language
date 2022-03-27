fn no_ownership_change(data:i32){
    println!("Hello: {}", data);
}

//Variables sotred in the stack: so there is NO OWNERSHIP OR CHANGE OF IT
pub fn ex_one(){
    println!("-----No Ownership Change-----");

    
    let data = 5;//Data has ownership of the value 5, until "y"
    let data2 = data;//Copy of data, but still in the stack
    println!("Data: {}", data);
    println!("Data2: {}", data2);
    no_ownership_change(data);

}//y

//Ownership Change
pub fn ex_two(){
    println!("-----Ownership Change-----");

    
    let v = vec![1,2,3];//Data is cretaed in the heap becvause vec! is a heap function. So this data now is on the heap
                        //and has an owner "v"
    println!("Data: {}", v[0]);
    let v2 = v;//Thenew owner of the heap data which "v" pointed to, is now "v2"
    println!("Data2: {}", v2[0]);
    //println!("Data: {}", v[0]);//prints "value borrowed here after move" error
                            //This is because rules determine that each variable in the heap can only have one owner at a time
    

}

//last ownership example
fn print_sum(v:Vec<i32>){
    //v is dropped and deallocated here
    println!("{}", v[0] + v[1]);
}
pub fn ex_three(){
    println!("-----Ownership Change 2-----");
    let mut v = Vec::new();// Creating the resource
    for  i in 100..1000 {
        v.push(i);
    }

    //at this point v is using no less than 3600 bytes of memory as one element = 4 bytes
    print_sum(v);//The ownership of the data in "v" is passed to the function
    println!("We are done.");
    //We no longer own nor have any control over "v"
    //It would be a compile time error to try and access "v" here
    println!("{}", v[0]);//prints "value used here after move" error
}