fn main(){
    let x : i32 = -42;
    let y : u32 = 100;
    println!("Signed Integer:{}",x);
    println!("Unsigned Integer:{}",y);
    //difference between i32(32 bits) and i64(64 bits)

    let e : i32 = 2147483647;
    let i : i64 = 9223372036854775807;
    println!("Maximum value of i32: {}",e);
    println!("Maximum value of i64: {}",i);
}