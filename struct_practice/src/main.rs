
fn print_values(tup: (&str, &str, i32)) {
    println!(
    "{} is a: {}, and is {} years old", 
    tup.0, tup.1, tup.2
    );
}

fn main() {

    let mut cat: (&str, &str, i32) = ("Chance", "Manecoon", 7);

    print_values(cat);

}
