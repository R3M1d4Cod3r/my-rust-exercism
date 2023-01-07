use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    
    //let word_input = Vec::new();
    //input.iter_mut().for_each( |x| { x.to_string().split(" ").for_each( | w | {  word_input.push(w);  } ); });

    
    let shared_data_result = Arc::new(Mutex::new( HashMap::<char, usize>::new() ) );

    let threads = Vec::new();

    for n in 0..worker_count{

        let mut data = shared_data_result.clone();

        threads.push( thread::spawn(move || {

            let word_map=HashMap::new();
            
            
        }));
    }

    return HashMap::new();
}
