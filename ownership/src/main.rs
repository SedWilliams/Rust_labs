fn main() {


    //Ownership is applied to heap allocated variables (mutable variables,custom types,inputs,etc)
    //CORRECTION: Only used with types that do not implement the Copy trait
    
    //Stack allocated variables do not have standard ownership rules
    //simply not true

    //Types that do not have the copy type operate under standard ownership
    
    //Functions can be owners

    /***************************
     * Copy types
     ***************************/
    let x: i32 = 10;
    //let y: u8 = x; invalid because literals are copied although not bound by regular ownership
    
    //works because the copied value is of the same type
    let y: i32 = x;

    //works because regular ownership rules are loopholed here
    println!("{}", x);

    /***************************
     * Non-copy types
     ***************************/

    //becomes invalid after y is initialized
    let x = String::from("Hello World");
    let y = x;

    //produces error because the heap allocated x is dropped after ownership is moved to y
    //println!("{}", x);
    
    
    /*
     * Copy types are standard literals like &str, int, float, boolean
     *
     * Non-copy type examples: pointers, String, structs
     *
     * Check the rust documentation for what types implement the copy trait.
     */
}
