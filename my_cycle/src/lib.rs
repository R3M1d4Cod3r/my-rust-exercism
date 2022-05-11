
pub mod my_cycle{

    pub struct MyCycle<I: Clone+Iterator>{
        pub iterator : I,
        pub clone : I,
        pub repeat : usize,
        pub index_clone : usize,
    }

    impl<I: Clone+Iterator> MyCycle<I>{
        pub fn new(iter:I,repeat:usize)->Self{
            let clone=iter.clone();
            Self{
                iterator:iter,
                clone: clone,
                repeat:repeat,
                index_clone : 1,
            }
        }

    }



impl<I: Clone+Iterator> Iterator for MyCycle<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let a = self.clone.next();
        if a.is_some(){
            a
        }
        else if self.index_clone != self.repeat {
            self.index_clone+=1;
            self.clone=self.iterator.clone();
            self.clone.next()
        }
        else{
            None
        }
    }
}
}



fn main(){
    let arr= my_cycle::MyCycle::new([1,2,3,4].iter(),1);
    println!("array:{:?} clone:{:?} index: {:?} n of clones: {:?}",arr.iterator,arr.clone,arr.index_clone,arr.repeat);
    for a in arr{
        println!("{:?} ", a);
    }
}