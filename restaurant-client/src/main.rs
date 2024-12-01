use std::thread;
use std::time::Duration;

// TODO: Make various restaurant client strategies to add variation
// 1. Create order, add items, delete items, check on status of items etc

fn thread_func(id: i32) {
    for i in 0..10 {
        println!("thread[{}]: hi number {i} from the spawned thread!", id);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn main() {
    let thread_count = 10;

    for i in 1..thread_count {
        thread::spawn(move || { thread_func(i) });
    }

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1000));
    }
}
