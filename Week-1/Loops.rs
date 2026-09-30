fn main(){
    // loop
    let mut count = 0;
    loop{
        println!("Count: {}",count);
        count += 1;
        if count == 3 {break;}
    }
    
    // while loop
    let mut n = 0;
    while n < 5{
        println!("{}",n);
        n += 1; 
    }
    
    // for loop
    for i in 1..5{
        println!("Number: {}", i);
    }
}