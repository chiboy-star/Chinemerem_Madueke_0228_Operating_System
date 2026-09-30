fn sum(numbers: &Vec<i32>) -> i32 {
    let mut total = 0;
    for num in numbers {
        total += num;
    }
    total
}

fn main() {
    let data = vec![1, 2, 3]; // `data` owns this vec
    let total = sum(&data);   // We pass a reference so `sum` borrows it
    println!("{:?} => {}", data, total); // Still valid here
}