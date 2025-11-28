use std::io;

fn main() {
    //vector holding all of the fibb sequence nums
    let mut seq: Vec<i32> = vec![0, 1, 1];

    //take user input for fibonacci to the nth
    let mut nth = String::new();
    println!("Input what power you would like to take the Fibonacci Sequence to: ");
    io::stdin().read_line(&mut nth).expect("Input something bozo");
    
    //turn it into an integer
    let nth: u32 = nth.trim().parse().expect("NaN");
    //loop as many times as the user wants
    let mut i = 0;
    while i < nth {
        
        let new_digit = seq.iter().rev().take(2).sum();

        seq.push(new_digit);
        
       //print findings
       println!("{}th digit of fibonacci: {}", i, new_digit);
       
       //update counter
       i += 1
    }
}
