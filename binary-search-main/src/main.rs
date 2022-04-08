use std::env;
use binary_search::find;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let key = args[1].clone();
    let mut buf=String::new();
    let mut array=vec![];
    loop{
        match std::io::stdin().read_line(&mut buf){
            Ok(0) => break,
            Ok(_x) => {array.push(buf.trim().to_string()); buf.clear();},
            Err(_)=>{println!("Errore Orrore");break;}
        }
    }
    println!("array : {:?} | key : {}",array,key);
    match find(array, key){
        None => println!("Non Trovato"),
        x => println!("Trovato alla posizione : {}",x.unwrap())
    }
}

