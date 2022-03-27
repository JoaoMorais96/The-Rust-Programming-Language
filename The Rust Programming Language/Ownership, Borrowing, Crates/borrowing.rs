//WITHOUT BORROWING
//Using a variable and returning it back to the original owner
//Here we are not getting the reference, but instead the original data on theheap
fn print_mull1(v: Vec<i32>) ->Vec<i32>{
    println!("{}", v[0]*v[1]);
    return v;
}

//WITH BORROWING (explicit references)
//Here wehave the references, so more than one variable is pointing to the same data on the heap
//Two variables reference the same data which is "vr" and "v"(main)
fn print_mull2(vr: &Vec<i32>){
    println!("{}", (*vr)[0]*(*vr)[1]);
}

//WITH BORROWING (explicit references)
//This is how we should actually do it
fn print_mull3(v: &Vec<i32>){
    println!("{}", v[0]*v[1]);
}

fn count_occurences(v2: &Vec<i32>, val:i32)->usize {
    v2.into_iter().filter(|&&x|x == val).count()
}
pub fn borrow_one(){
    println!("-----Borrowing-----");

    let mut v = Vec::new(); //creating the resource

    for i in 100..1000{
        v.push(i);
    }
     //at this point v is using no less than 3600 bytes of memory as one element = 4 bytes

     //transfer ownership to print_mull() and get it back after they are done
     v = print_mull1(v);

     //Now we again own and control "v"
     println!("(1) We still have v: {}, {} ...", v[0], v[1]);//Prints "(1) We still have v: 100, 101 ..."

     //Take a reference to "v" (borrow it) and pass this reference to print_mul2
     print_mull2(&v);

     //"v" is still completly ours
     println!("(2) We still have v: {}, {} ...", v[0], v[1]);//Prints "(1) We still have v: 100, 101 ..."

     //same as in above example
     print_mull2(&v);
     println!("(3) We still have v: {}, {} ...", v[0], v[1]);//Prints "(1) We still have v: 100, 101 ..."

     println!("\n Another example");

     let v2 = vec![2,9,3,1,3,2,5,5,2];
     //borrowing v2 for the iteration and item is the reference

     for &item  in &v2 {
         //the first borrow is still active
         //we borrow the second time here
         let res = count_occurences(&v, item);
         println!("{} is repeated {} times", item, res);
     }
}