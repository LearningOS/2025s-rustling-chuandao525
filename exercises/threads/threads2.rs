// threads2.rs
//
// Building on the last exercise, we want all of the threads to complete their
// work but this time the spawned threads need to be in charge of updating a
// shared value: JobStatus.jobs_completed
//
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a
// hint.


use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: AtomicU32,
}

fn main() {
    let status = Arc::new(JobStatus {
        jobs_completed: AtomicU32::new(0),
    });
    let mut handles = vec![];

    // 生成10个线程
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 原子递增计数器
            status_shared.jobs_completed.fetch_add(1, Ordering::SeqCst);
        });
        handles.push(handle);
    }

    // 等待所有线程完成并打印结果
    for handle in handles {
        handle.join().unwrap();
    }
    
    // 最终打印总和
    println!(
        "jobs completed {}",
        status.jobs_completed.load(Ordering::SeqCst)
    );
}