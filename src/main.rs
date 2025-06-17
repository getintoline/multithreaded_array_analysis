// Author: River Moshkina
// Lab: Multithreaded Array Analysis in Rust
// Description: Counts zeros, positives, and negatives using single- and multi-threading with timing and correctness checks.

use rand::Rng;
use std::thread;
use std::time::Instant;

/// Generates a vector of random i32 values in the range [-range, +range]
fn generate_data(size: usize, range: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen_range(-range..=range)).collect()
}

/// Single-threaded counter
fn count_elements_single(data: &[i32]) -> (usize, usize, usize, u128) {
    let start = Instant::now();
    let (mut zeros, mut pos, mut neg) = (0, 0, 0);

    for &x in data {
        if x == 0 {
            zeros += 1;
        } else if x > 0 {
            pos += 1;
        } else {
            neg += 1;
        }
    }

    let duration = start.elapsed().as_millis();
    (zeros, pos, neg, duration)
}

/// Multi-threaded counter using user-defined number of threads
fn count_elements_multi(data: &[i32], num_threads: usize) -> (usize, usize, usize, u128) {
    let start = Instant::now();
    let chunk_size = data.len() / num_threads;
    let mut handles = vec![];

    for i in 0..num_threads {
        let chunk = if i == num_threads - 1 {
            &data[i * chunk_size..]
        } else {
            &data[i * chunk_size..(i + 1) * chunk_size]
        };

        // Send chunk to thread
        let chunk = chunk.to_vec();
        let handle = thread::spawn(move || {
            let (mut zeros, mut pos, mut neg) = (0, 0, 0);
            for &x in &chunk {
                if x == 0 {
                    zeros += 1;
                } else if x > 0 {
                    pos += 1;
                } else {
                    neg += 1;
                }
            }
            (zeros, pos, neg)
        });

        handles.push(handle);
    }

    let mut zeros = 0;
    let mut pos = 0;
    let mut neg = 0;

    for handle in handles {
        let (z, p, n) = handle.join().unwrap();
        zeros += z;
        pos += p;
        neg += n;
    }

    let duration = start.elapsed().as_millis();
    (zeros, pos, neg, duration)
}

fn main() {
    // Parameters
    let array_size = 100_000_000;
    let value_range = 1000;
    let thread_count = 4;

    println!("Generating array with {} elements in range [-{}, {}]...", array_size, value_range, value_range);
    let data = generate_data(array_size, value_range);

    println!("Starting single-threaded count...");
    let (zeros_single, pos_single, neg_single, time_single) = count_elements_single(&data);
    println!("Single-threaded: {} zeros, {} positives, {} negatives in {} ms", zeros_single, pos_single, neg_single, time_single);

    println!("Starting multi-threaded count with {} threads...", thread_count);
    let (zeros_multi, pos_multi, neg_multi, time_multi) = count_elements_multi(&data, thread_count);
    println!("Multi-threaded:  {} zeros, {} positives, {} negatives in {} ms", zeros_multi, pos_multi, neg_multi, time_multi);

    assert_eq!((zeros_single, pos_single, neg_single), (zeros_multi, pos_multi, neg_multi));
    println!("✅ Results match! 🎯");

    if time_multi < time_single {
        println!("🎉 Multithreaded version was faster by {} ms!", time_single - time_multi);
    } else {
        println!("😅 Multithreaded version was slower by {} ms... (try increasing array size or tuning thread count)", time_multi - time_single);
    }
}
