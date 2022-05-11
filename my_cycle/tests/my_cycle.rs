use my_cycle::my_cycle::MyCycle;
fn process_valid_case(array1:&[i32], repeat:usize,array2: &Vec<i32>) {
    
    let mut  a = Vec::new();
    for _ in 0..repeat{
    for i in array1{
        a.push(i);
    }}
    assert_eq!(a,array2);
}

#[test]
fn normal_test(){
    
    let a = MyCycle::new([1,2,3,4].iter(),2);

    let mut b= Vec::new();
    for x in a{
        b.push(x);
    }
    
    process_valid_case(&[1,2,3,4],2,&b )

}