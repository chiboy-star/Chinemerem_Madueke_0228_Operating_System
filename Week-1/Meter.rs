use std::io;

fn main() {
    // Prompt the user to enter their energy consumption
    println!("Please enter your energy consumption in kWh:");

    // Read the user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into a floating-point number
    let consumption: f64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a number.");
            return;
        }
    };

    // Determine the rate per unit based on usage thresholds
    
    let rate_per_unit = if consumption > 200.0 {
        30.0 // ₦30 per unit for usage above 200 kWh[cite: 2]
    } else if consumption > 100.0 {
        25.0 // ₦25 per unit for usage above 100 kWh[cite: 2]
    } else {
        20.0 // Base rate of ₦20 per kWh[cite: 2]
    };

    // Calculate the final bill
    let total_bill = consumption * rate_per_unit;

    // Display the result
    println!("Energy Consumed: {} kWh", consumption);
    println!("Rate Applied: ₦{} per kWh", rate_per_unit);
    println!("Total Electricity Bill: ₦{}", total_bill);
}