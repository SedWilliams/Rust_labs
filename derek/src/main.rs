use std::io;

fn main() {
    
    println!("Hello Shedrick");

    let mut name: String = String::new();

    io::stdin().read_line(&mut name)
        .expect("No Input");

    println!("This is what you input: {}", name);

    let mut name: u32 = name.trim().parse()
        .expect("Could not convert type");

    name = name + 1;

    println!("{}", name);
}
