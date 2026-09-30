fn main() {
    // Set the original bill amount
    let original_bill = 12000.0;
    
    
    let discount = if original_bill > 10000.0 {
        15 // 15% discount for bills above ₦10,000
    } else if original_bill > 5000.0 {
        10 // 10% discount for bills above ₦5,000
    } else {
        0  // No discount otherwise
    };

    //the final bill amount
    let final_bill = original_bill - (original_bill * (discount as f64 / 100.0));

    //Print the output
    println!("Original Bill: ₦{}", original_bill);
    println!("Discount Applied: {}%", discount);
    println!("Final Bill: ₦{}", final_bill);
}