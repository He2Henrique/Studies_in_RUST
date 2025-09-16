
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

    //in rust to u pass references that is references to just read or mutable references

    let mut str = String::from("hello");

    pushing_world(&mut str); 

    let num: usize = takes_len(&str);

    print!("{num}");

    
}

fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn pushing_world(s: &mut String){
    s.push_str("World");

}

fn takes_len(s: &String)-> usize{
    return s.len(); 
}
