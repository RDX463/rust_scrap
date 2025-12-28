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
//================================================================
//Tupple Datatype
    let human : (String , i32 , bool) = ("Rohan".to_string(),22,true); //added function to convert
                                                                       //into String 
    println!("My info {:?}", human);
    let mix_tuple = ("Bats",5,false,[1,2,4,5,8]);
    println!("Mix tuple is {:?}",mix_tuple);                //prints all datatype including array  
//=================================================================
//Slices
}
