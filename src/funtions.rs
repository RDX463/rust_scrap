fn main(){
    hello_world();
    tell_height(182);
    human_info("Rohan",22,172.0);
    let x = {                     //Ininitalizing a statement
        let price = 5;
        let quantity = 10;
        price * quantity
    };
    println!("Result is {}",x);
    //add(4,6);
    let y = add(4,6);
    println!("Value of y is: {}",y);
    println!("Value from 'add()' is {}.",add(5,6));

    //calling bmi function
    let w = 55.6;
    let h = 1.65;
    let bmi = calculate_bmi(w,h);
    println!("My calculated BMI is {:.2}",bmi);
}

//Hoisting-call function anywhere
fn hello_world(){
    println!("Hello Rust!");
}

fn tell_height(height:u32){
    println!("My height is {}.cm",height);
}

fn human_info(name: &str,age:u32,heights:f32){
    println!("My name is {}. I'm {} years old. And my height is {}.cm",name,age,heights);
}

//function returning a value.
fn add(a:i32,b:i32)->i32{
    a+b
}

fn calculate_bmi(weight_kg:f64,height_m:f64)->f64{
    weight_kg/(height_m * height_m)
}
