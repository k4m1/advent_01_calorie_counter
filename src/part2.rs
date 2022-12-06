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

    // Sort the `sums` vector in descending order
    sums.sort_by(|a, b| b.cmp(a));

    // Take the first 3 elements from the `sums` vector
    let top_3 = &sums[..3];

    // Find the sum of the first 3 elements in the `sums` vector
    let sum: i32 = top_3.iter().sum();

    // Print the sum of the first 3 elements in the `sums` vector
    println!("Sum of top 3: {}", sum);
}
