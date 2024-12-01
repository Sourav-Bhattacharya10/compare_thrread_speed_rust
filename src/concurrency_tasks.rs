use rand::Rng;
use std::thread;

pub fn generate_random_array() -> [i16; 10000] {
    let mut rng = rand::thread_rng();

    let arr: [i16; 10000] = [rng.gen(); 10000];

    arr
}

pub fn find_max(arr: &[i16]) -> Option<i16> {
    const MIN_NUM_IN_ARRAY: usize = 2;

    if arr.len() <= MIN_NUM_IN_ARRAY {
        return arr.iter().cloned().max();
    }

    let mid: usize;
    if arr.len() % 2 == 0 {
        mid = arr.len() / 2;
    } else {
        mid = (arr.len() / 2) + 1;
    }

    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let handle_left = thread::spawn(move || find_max(&left));
    let handle_right = thread::spawn(move || find_max(&right));

    let max_left = handle_left.join().unwrap()?;
    let max_right = handle_right.join().unwrap()?;

    Some(max_left.max(max_right))
}

// pub async fn find_max(array: Arc<[i16]>, start: usize, end: usize) -> i16 {
//     if start == end {
//         // Base case: single element
//         return array[start];
//     }

//     if start + 1 == end {
//         // Base case: two elements
//         return i16::max(array[start], array[end]);
//     }

//     let mid = (start + end) / 2;

//     // Spawn asynchronous tasks for both halves
//     let array_clone = array.clone();
//     let left_task = task::spawn(async move { find_max(array_clone, start, mid).await });

//     let array_clone = array.clone();
//     let right_task = task::spawn(async move { find_max(array_clone, mid + 1, end).await });

//     // Wait for both tasks to complete
//     let left_max = left_task.await.unwrap();
//     let right_max = right_task.await.unwrap();

//     // Return the maximum of the two
//     i16::max(left_max, right_max)
// }
