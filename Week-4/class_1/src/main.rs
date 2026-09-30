use std::thread;

fn main() {

    //let name  = String::from("Chinemerem");
    //let greet = || println!("Hell, {}!", name);
    //greet();
    //println!("Still usable: {}", name);
    //let owned = move || println!("Hello again, {}", name);
    //owned();
    //println!("{}",name); //This will not compile: moved

    let handle = thread::spawn(||{
        for i in 1..5{
            println!("hi number {} from the spawned thread!", i);
        }
    });

    let handle2 = thread::spawn(||{
        for i in 1..5{
            println!("hi number {} from the 2nd spawned thread!", i);
        }
    });

    let handle3 = thread::spawn(||{
        for i in 1..3{
            println!("hi number {} from the 3rd spawned thread!", i);
        }
    });
    
    handle.join().unwrap();
    println!("done in main 1");
    handle2.join().unwrap();
    println!("done in main 2");
    handle3.join().unwrap();
    println!("done in main 3");
}
