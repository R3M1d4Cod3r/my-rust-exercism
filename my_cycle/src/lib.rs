
pub mod my_cycle{
    struct MyCycle<I: Clone+Iterator>{
        iterator : I,
        repeat : usize,
    }

    impl<I: Clone+Iterator> MyCycle<I>{
        pub fn new(self,iter:I,repeat:usize)->Self{

            let mut result=iter.clone();
            for _ in 0..repeat{
                result =result.clone();
            } 
            Self{
                iterator:result,
                repeat:repeat
            }
        }

    }
}