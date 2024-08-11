//  Primitive Data Types
//  int float bool char

// Integer
// Rust Signed Integer Types (Imzoli butun son turlari)
// Rust Unsigned Integer Types (Imzosiz butun son turlari)
// Signed   i8 i16 i32 i64 i128
// Unsigned u8 u16 u32 u64 u128

fn main(){
    let x: u64 = 45;
    let y: u64 = 100;
    println!("Signed integer {x}");
    println!("UnSigned integer {y}");

    if x > y {
        println!("y dan x katta")
    } else {
        println!("x dan y katta")
    }
}