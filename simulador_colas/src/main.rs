use std::{env::{args}, thread::{spawn, self}, sync::{Arc, Mutex}, time::Duration};

use rand::Rng;

fn main() {
    let args: Vec<String> = args().collect();

    let mut queues_number = 5;

    if args.len() > 1 && args[1].parse::<usize>().unwrap_or_default() > 0 {
        queues_number = args[1].parse().unwrap_or_default();
    } else {
        println!("No number of queues has been passed as an argument, so there will be {} queues by default.", queues_number);
    }

    let queues: Arc<Mutex<Vec<i32>>> = Arc::new(Mutex::new(Vec::new()));

    for i in 0..queues_number {
        let queues_clone = Arc::clone(&queues);

        queues_clone.lock().unwrap().push(0);

        spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(500..5000)));

                if queues_clone.lock().unwrap()[i] > 0
                {
                    queues_clone.lock().unwrap()[i] -= 1;
                }
            }
        });
    }

    let queues_clone = Arc::clone(&queues);

    spawn(move || {
        loop {
            thread::sleep(Duration::from_millis(rand::thread_rng().gen_range(100..1000)));

            queues_clone.lock().unwrap()[rand::thread_rng().gen_range(0..queues_number)] += 1;
        }
    });

    let queues_clone = Arc::clone(&queues);

    loop {
        thread::sleep(Duration::from_millis(500));

        for i in 0..queues_number {
            println!("Number of customers in queue {}: {}", i + 1, queues_clone.lock().unwrap()[i]);
        }

        println!();
    }
}
