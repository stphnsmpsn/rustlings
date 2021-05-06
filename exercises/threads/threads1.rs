// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

// I AM DONE 2021-05-06 by stphnsmpsn

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let status_shared = status.clone();
    thread::spawn(move || {
        for _ in 0..10 {
            thread::sleep(Duration::from_millis(250));
            {
                status_shared.lock().unwrap().jobs_completed += 1;
            }
        }
    });

    // todo: I'm not convinced here when the lock gets unlocked...
    //  is it held for the whole sleep? (the scope each loop iteration)
    //  or is its scope only during the evaluation
    while status.lock().unwrap().jobs_completed < 10 {
        println!("waiting... ");
        thread::sleep(Duration::from_millis(500));
    }

    // todo: is this safer? it's more obvious that the lock is only held for an instant but
    //  I'm not sure if it's any different than the above
    // let mut n = 0;
    // while n < 10 {
    //     println!("waiting... ");
    //     thread::sleep(Duration::from_millis(500));
    //     {
    //         n = status.lock().unwrap().jobs_completed;
    //     }
    // }
}
