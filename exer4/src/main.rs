fn main(){
    let a = vec![1,2,3,4,5];
    // let mut i = 0;
    // let c = 0;
    
    let (a, c) = test(a);
    println!("{:?}, {}", a, c);
}

pub fn test(mut a: Vec<u8>) -> (Vec<u8>, i32) {
    let mut b:Vec<u8>  = Vec::new();
    let mut c:u8 = 0;
    loop {
        if a.len() == 0 { break; }
        let d = a.pop().unwrap();
        c = c+d;
        b.push(d);
    }
    (b, c as i32)
}