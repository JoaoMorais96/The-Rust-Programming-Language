
pub fn main2(){
    println!("Starting Language Fund. II");

    //if statements
    let x:bool = true;
    if x {
        println!("yeah, we're true");
    } else {
        println!("Booo, it's false")
    }

    let y = 3;
    if (y == 4) && (x == true) {
        println!("we have a 4");
    } else if (y == 3) || (x == false){
        println!("we have a 3")
    } else {
        println!("not a 3 or a 4");
    }

    //loops
    let mut x = 5;
    let mut not_done = true;
    println!("Begginning of while loop");
    while not_done {
        x+=x-3;
        println!("{}",x);
        if x % 5 == 0 {
            not_done = false;
        }
    }

    println!("Begginning of for loop");
    for x in 0..20 {
        println!("{}",x);
    }

    //match statements
    println!("Begginning of match statements");
    let x = "Apple";
    match x {
        "Apple" => println!("Great Brand"), //if it is a match with "Apple", print "Great brand"
        "Linux" => println!("Open Source"), //if it is a match with "Linux", print "Open Source"
         _ => println!("Open Source")//if it doesn't match with any of the possible matches
    };

    let area_code = 206;
    let area  =  match area_code {
        206 => "Seattle",
        318 => "Louisiana",
        200..=300 => "Washington State",
        _ => "Invalid"
    };
    println!("The area for {} is {}", area_code, area);

}

























