use rand::Rng; // Import the Rng trait
use serde::Serialize;
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Debug)]
struct MatrixPair {
    a: Vec<Vec<f64>>,
    b: Vec<Vec<f64>>,
}

fn main() {
    // Define the matrix size (500 or 1000 for large sizes)
    let size = 1000;

    // Generate the random matrices
    let matrices = generate_matrix_pair(size);

    // Save the matrices to a binary file
    write_to_file("matrix_data.json", &matrices).expect("Failed to write matrices to file");
    println!("Done");
}

fn generate_matrix_pair(size: usize) -> MatrixPair {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Generate two random matrices
    let mut generate_matrix = || {
        (0..size)
            .map(|_| {
                (0..size)
                    .map(|_| rng.gen_range(0.0..1.0)) // Generate a random float between 0.0 and 1.0
                    .collect()
            })
            .collect()
    };

    MatrixPair {
        a: generate_matrix(),
        b: generate_matrix(),
    }
}

fn write_to_file(filename: &str, data: &MatrixPair) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let json = serde_json::to_string(data)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}