// Calculation with rust
// fn main (){
//     let a: f64 = 66.0;
//     let b: f64 = 9.0;

//     let result = a / b;
//     println!(" The sum of the number is {}", result)
// }

// the result show have decimal, so if you are expecting decimal you will have to use f64
// you have to make the values decimal too

// What happen when you try to buy something with you card

fn main() {
    let balance: f64 = 59.0;
    let shoes: f64 = 58.1;
    let bonus: f64 = 0.9;

    let result = balance - (shoes + bonus); // BODMAS know will help you with this 
    if balance > shoes  {
        println!("you have paid and your balance is {}", result);
    } else {
        println!("You are broke");
    }
}

