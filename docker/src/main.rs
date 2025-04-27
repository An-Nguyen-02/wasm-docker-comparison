use std::fs::{File};
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};
use serde_json;
use std::time::Instant;
use sysinfo::{System, Pid};


#[derive(Serialize, Deserialize, Debug)]
struct MatrixPair {
    a: Vec<Vec<f64>>,
    b: Vec<Vec<f64>>,
}

fn main() {
    println!("Starting matrix processing...");
    let start_total = Instant::now();
    let mut sys = System::new_all();
    let pid = Pid::from(std::process::id() as usize);

    // Start system measurement for CPU and memory usage
    let start_sys = Instant::now();
    // sys.refresh_process(pid);
    // let process = sys.process(pid).expect("Failed to get process info");
    // let initial_cpu = process.cpu_usage();
    // let initial_memory = process.virtual_memory();
    let sys_init_time = start_sys.elapsed();

    // Start reading matrix from file
    let start_read = Instant::now();
    let process_matrix = read_from_file("matrix_data.json");
    let read_time = start_read.elapsed();

    // Start matrix multiplication process
    let start_compute = Instant::now();
    let result = matrix_multiply(&match process_matrix {
        Ok(matrix) => matrix,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    });
    let compute_time = start_compute.elapsed();

    // Result writing to a file
    let start_result_write = Instant::now();
    let _ = write_result_to_file("result.json", &result);
    let result_write_time = start_result_write.elapsed();

    let total_time = start_total.elapsed();

    // Taking CPU and memory usage
    let start_sys1 = Instant::now();
    sys.refresh_process(pid);
    let process = sys.process(pid).expect("Failed to get process info");
    let final_cpu = process.cpu_usage();
    let final_memory = process.virtual_memory();

    let cpu_usage = final_cpu;
    let memory_used_b = final_memory;
    let memory_used_mb = memory_used_b as f64 / (1000.0 * 1000.0);
    let sys_init_time1 = start_sys1.elapsed();
    
    // Print results
    println!("Disk Read Time: {:.5} s", read_time.as_secs_f64());
    println!("Computation Time: {:.5} s", compute_time.as_secs_f64());
    println!("Result Write Time: {:.5} s", result_write_time.as_secs_f64());
    println!("Total Execution Time: {:.5} s", total_time.as_secs_f64());
    println!("CPU Usage During Execution: {:.5}%", cpu_usage);
    println!("Memory Usage Increase: {:.5} MB", memory_used_mb);
    println!("Overhead time due to CPU and Memory measurement: {:5} s", (sys_init_time1 + sys_init_time).as_secs_f64());

    println!("Matrix processing completed. Results written to 'result.json'.");
}

fn write_result_to_file(filename: &str, data: &Vec<Vec<f64>>) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    let json = serde_json::to_string(data)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

fn read_from_file(filename: &str) -> std::io::Result<MatrixPair> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let matrices: MatrixPair = serde_json::from_str(&contents)?;
    Ok(matrices)
}

fn matrix_multiply(matrix: &MatrixPair) -> Vec<Vec<f64>> {
    let size = matrix.a.len();
    let mut result = vec![vec![0.0; size]; size];

    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                result[i][j] += matrix.a[i][k] * matrix.b[k][j];
            }
        }
    }

    result
}