
use std::io;
// use is a way to reduce the keywords to call a data from other module.
use crate::folder::external_mod::section::Retangle;

//in rust u need to specify the module tree if u dont create a file that import submudules
// u cant acess the code as the follwing case.
mod folder{
    pub mod external_mod;
}

// the path to the source code is src/folder/external_mod{section{Retangle}}
// mod represent scopes like directories it can be a folder, file or a fragmant of the code.


fn main() {
    println!("Hello, world!");

    let simple = Retangle::create(12,32);

    print!("{:?}",simple);

    let condition = true;
    let number = if condition 
        { println!("\n u get a sum!"); 
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
