# Multithreaded Array Analysis

Alright y’all, this here Rust program counts how many zeros, positives, and negatives are sittin’ in a big ol’ array. You can run it plain single-threaded, or spice it up with multithreading — just holler how many threads you want when it asks.

It times both ways and makes sure the numbers match up. Multithreadin’ oughta be faster, no doubt.

Run it with `cargo run --release`. Tested on macOS, Rust 1.85.0

Here’s a sample of what you’ll see:

Single-threaded counts: zeros=49888, positives=49979203, negatives=49970909
Multithreaded counts: zeros=49888, positives=49979203, negatives=49970909
Single-threaded time: 115 ms
Multithreaded time: 89 ms


Windows executable? Maybe down the road.

That’s all, folks.
