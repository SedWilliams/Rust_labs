/***************************************
 * Examples of using structs in Rust
 ***************************************/

//traditional struct
struct PointStruct {
    x: f64,
    y: f64,
}

//tuple struct
struct PointTuple(f64, f64);

//struct method
impl PointStruct {
    fn print_point(&self) -> f64 {
        println!("PointStruct coordinates: ({}, {})", self.x, self.y);
    }
}

fn main() {

    let struct_point = PointStruct { x: 3.0, y: 4.0 };
    
    // use a method defined for a struct
    struct_point.print_point();

}
