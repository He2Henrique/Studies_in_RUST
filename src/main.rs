
use std::io;
fn main() {
    println!("Hello, world!");

    let condition = true;
    let number = if condition 
        { println!("u get a sum!"); 
            sum(2, 2)} else { 6 };


    println!("{number}");


    loop {
        let mut resposta = String::new();

        io::stdin()
            .read_line(&mut resposta)
            .expect("Somethig goes wrong");

        println!("{resposta}");

        if resposta.trim() == "esc"{
            break;
        }
    }

    
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}
