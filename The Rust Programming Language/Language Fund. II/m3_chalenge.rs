//Instructions:
    //-Check if a number can be devided by 2 or not
    //-Then take the result, store it in a variable
    //- Then use the result to print (using match): 
    //                           -"Yes" if it can be devided 
    //                           -"No" if it can't
pub fn m3_challenge_main(){
    println!("Start of challenge M3!");
    let number_test = 20;
    let devisible_by_two:bool ;
    if number_test%2 == 0 {
        devisible_by_two = true;
    } else {
        devisible_by_two = false;
    }

    let response = match devisible_by_two {
        true => "is divisible by 2",
        false => "is not divisible by 2"
    };
    println!("The number {} {}", number_test, response);

}