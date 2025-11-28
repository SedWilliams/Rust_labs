use std::io;
//the conversions (incorrect, update later)
// C -> F (x*1.8) + 32)
// F -> C (x*(5/9)) - 32

//take in temp in fahrenheight and output in celcius
fn to_celcius(temp_f: i64) -> i64 {
    let temp_c =  ((temp_f - 32) * 5/9);
    println!("{} in fahrenheight is {} in celcius", temp_f, temp_c);
    temp_c
}

//take in temp in celcius and output fahrenheight
fn to_fahrenheight(temp_c: i64) -> i64 {
    let temp_f = ((temp_c * 9/5) + 32);
    println!("{} in celcius is {} in fahrenheight", temp_c, temp_f);
    temp_f
}

fn main() {
    
    //take in user choice 1 for F -> C and 2 for 200 C -> F
    let mut conversion_choice = String::new();
    println!("1) Fahrenheight -> Celcius");
    println!("2) Celcius -> Fahrenheight");
    println!("Which conversion would you like to make: ");
    io::stdin().read_line(&mut conversion_choice).expect("Not a valid input!");
    
    //turn input into integer 1 or 2
    let conversion_choice: i64 = conversion_choice.trim().parse().expect("NaN");
    
    //if 1 convert from f to c
    //if 2 convert from c to f
    if conversion_choice == 1 {
        let mut temp = String::new();
        println!("Input temperature: ");
        io::stdin().read_line(&mut temp).expect("Not a valid temperature.");
        let mut temp: i64 = temp.trim().parse().expect("NaN");
        println!("Heard: {}", temp);

        to_celcius(temp);

    } else if conversion_choice == 2 {
        
        let mut temp = String::new();
        println!("Input temperature: ");
        io::stdin().read_line(&mut temp).expect("Not a valid temperature.");

        let mut temp: i64 = temp.trim().parse().expect("NaN");
        println!("Heard: {}", temp);

        to_fahrenheight(temp);
    } else {
        println!("Input a valid number");
    }
}














