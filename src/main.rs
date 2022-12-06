use std::fs::File; // Import the `File` type from the `fs` module in the `std` crate
use std::io::{BufReader, BufRead}; // Import the `BufReader` and `BufRead` traits from the `io` module in the `std` crate

fn main() {
    // Open the file in read-only mode
    let file = match File::open("input.txt") {
        Ok(file) => file, // If the file is successfully opened, store it in the `file` variable
        Err(error) => { // If there was an error opening the file, handle the error
            println!("Error opening file: {}", error); // Print an error message
            return; // Return from the function
        }
    };
    let reader = BufReader::new(file); // Create a new `BufReader` to read the file

    let mut sum = 0; // Declare and initialize a `sum` variable to 0
    let mut sums = Vec::new(); // Declare and initialize an empty `sums` vector to store the sums

    // Loop through each line in the file
    for line in reader.lines() {
        // Read the line into a string
        let line = match line {
            Ok(line) => line, // If the line is successfully read, store it in the `line` variable
            Err(error) => { // If there was an error reading the line, handle the error
                println!("Error reading line: {}", error); // Print an error message
                continue; // Skip to the next iteration of the loop
            }
        };

        // If the line is empty, write the sum to the array and reset the sum variable
        if line.is_empty() {
            sums.push(sum); // Push the current sum to the `sums` vector
            sum = 0; // Reset the `sum` variable to 0
            continue; // Skip to the next iteration of the loop
        }

        // Convert the string to an integer
        let num = match line.parse::<i32>() {
            Ok(num) => num, // If the string is successfully parsed to an integer, store it in the `num` variable
            Err(error) => { // If there was an error parsing the integer, handle the error
                println!("Error parsing integer: {}", error); // Print an error message
                continue; // Skip to the next iteration of the loop
            }
        };

        // Add the integer to the running sum
        sum += num;
    }

    // Find the largest integer in the sums array
    let max = match sums.iter().max() {
        Some(max) => max, // If the `sums` vector is not empty, extract the largest integer and store it in the `max` variable
        None => { // If the `sums` vector is empty, handle the error
            println!("Sums array is empty"); // Print an error message
            return; // Return from the function
        }
    };

    // Print the largest integer
    println!("Max: {}", max);
}

