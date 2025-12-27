//compound data types
fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers in array are {:?}",numbers);
    let fruits : [&str; 4] = ["Mango","Apple","Banana","Kiwi"];
    println!("Fruits array {:?}",fruits);
    println!("First Fruit in Array {}",fruits[0]);
    println!("Second Fruit in Array {}",fruits[1]);
    println!("Third Fruit in Array {}",fruits[2]);
    println!("Fourth Fruit in Array {}",fruits[3]);
}
