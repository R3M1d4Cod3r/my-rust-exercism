use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_map: HashMap<&str, u32> = HashMap::new();
    for word in magazine{
        match magazine_map.get(word){
            None => { magazine_map.insert(word,1);},
            Some(number) => {magazine_map.insert(word,number+1);},
        }
    }

    for word in note{
        match magazine_map.get(word){
            None =>  {return false; } , 
            Some(number) => {if number -1 == 0 {magazine_map.remove(word);} else {magazine_map.insert(word,number-1);}},
        }
    }
    
    true
}
