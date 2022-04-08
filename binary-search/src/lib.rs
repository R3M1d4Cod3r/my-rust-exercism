


pub fn find<T,U>(array: U, key: T) -> Option<usize> where T : Ord, U: AsRef<[T]> {
    let array=array.as_ref();
    if array.len()==0 {return None;}
    if array.len()==1 {return if key == array[0] { Some(0) } else{ None }}
    let head = if (array.len()-1)%2==0 { (array.len()-1)/2 } else { (array.len()-1)/2+1 };

    if key == array[head] { return Some(head) };
    if key < array[head]{
        return find(&array[0..head],key);
    }
    else {
        let tmp:Option<usize> = find( &array[head.. array.len()],key);
        return if tmp == None { tmp } else { Some( head + tmp.unwrap() )}
    }
    
}







/*
    //let head =if array.len() == 1 { 0 } else{ if array.len()%2==0 { array.len()/2} else { (array.len()-1)/2 }};


    -----Metodo Stupido Stile C-----
fn binary_find(array: &[i32], key: i32, left : usize, right:usize ) -> Option<usize> {
    println!(" left : {}  right : {}",left,right);
    if left==right {return if key==array[left] {Some(left)} else {None}};

    let head= head_index(left, right);
    println!("head : {}",head);
    if key == array[head] { return Some(head) };

    if key < array[head] { 
        binary_find(array, key, left, head-1)
    } 
    else { 
        binary_find(array, key, head+1, right)
    }
}
fn head_index( left : usize, right:usize)-> usize{
    if right==left {
        return left;
    }
    println!("{}",(right-left)/2);
    let i =right-left;
    return if (right-left)%2==0 { left+(right-left)/2 } else { left+(right-left)/2+1 } ;
}


fn _main(){
    if find(&[1, 3, 5, 8, 13, 21, 22, 34, 55, 89, 144, 233], 13) == Some(4){
        println!("trovato");
    }
    else {
        println!("Non Trovato")
    }

    //binary_find(array, key, 0, array.len()-1)
}*/