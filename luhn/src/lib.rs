/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    
    if code.trim().len() <= 1 {
        println!("Code : {} not valid", code);
        return false;
    }

    let mut vec_c: Vec<char> = Vec::new();
    for c in code.chars() {
        if c.is_ascii_digit(){
            vec_c.push(c);
        }
        else if ! c.is_ascii_whitespace(){
            return false;
        }
    }
    println!("{:?}",vec_c);

    let mut i = vec_c.len();
    let mut somma = 0;
    let mut flag = false;

    while i > 0 {
        i -= 1;
        if flag {
            let doppio = vec_c[i].to_digit(32).unwrap() * 2;
            if doppio > 9 {
                somma += doppio - 9
            } else {
                somma += doppio
            }
        } else {
            somma += vec_c[i].to_digit(32).unwrap();
        }
        flag = !flag;
    }

    somma % 10 == 0
}

fn main(){
    println!("{}",is_valid("055 444 285"));
    return ;
}