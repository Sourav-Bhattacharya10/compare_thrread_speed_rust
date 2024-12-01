mod concurrency_tasks;

use std::time::Instant;
use tokio;

use concurrency_tasks::{find_max, generate_random_array};

#[tokio::main]
async fn main() {
    let arr = generate_random_array();
    let now = Instant::now();
    println!("Starting timer now");
    let max = find_max(&arr);
    println!("Finshed in {} milliseconds", now.elapsed().as_millis());
    println!("The max in the array is : {:?}", max);
}
