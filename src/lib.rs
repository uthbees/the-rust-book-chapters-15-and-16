use num_format::{Buffer, Locale};
use rand::Rng;
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::Duration;
use std::{io, thread};

const THREAD_COUNT: i8 = 5;
const MIN_THREAD_SLEEP_MILLIS: u64 = 250;
const MAX_THREAD_SLEEP_MILLIS: u64 = 3000;
const INITIAL_BALANCE: i64 = 100_000_000;
const MIN_TRANSACTION_AMOUNT: i64 = -2_000_000;
const MAX_TRANSACTION_AMOUNT: i64 = 2_000_000;

/// Simulates a heavily used company bank account, with multiple threads
/// handling (fake) incoming requests for deposits or withdrawals.
pub fn bank_simulation() {
    let balance = Arc::new(Mutex::new(INITIAL_BALANCE));
    println!(
        "Balance initialized to {}.",
        balance.lock().expect("Threads should not panic")
    );

    let mut thread_handles = vec![];
    let running = Arc::new(Mutex::new(true));

    for _ in 0..THREAD_COUNT {
        thread_handles.push(run_simulation_thread(&balance, &running));
    }

    println!("Simulation is running - press enter to quit.");
    // Wait until the user sends some input.
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    println!("Wrapping up the last few transactions...");

    let mut running = running.lock().expect("Threads should not panic");
    *running = false;
    // Release the lock so that we don't deadlock.
    drop(running);

    for handle in thread_handles {
        handle.join().expect("Threads should not panic");
    }

    println!(
        "Simulation finished. The final balance was {}.",
        format_money(*balance.lock().expect("Threads should not panic"))
    );
}

/// Run a thread of the bank simulation.
fn run_simulation_thread(balance: &Arc<Mutex<i64>>, running: &Arc<Mutex<bool>>) -> JoinHandle<()> {
    let balance = Arc::clone(balance);
    let running = Arc::clone(running);

    thread::spawn(move || {
        let mut rng = rand::thread_rng();

        while *running.lock().expect("Threads should not panic") {
            thread::sleep(Duration::from_millis(
                rng.gen_range(MIN_THREAD_SLEEP_MILLIS..=MAX_THREAD_SLEEP_MILLIS),
            ));

            let mut locked_balance = balance.lock().expect("Threads should not panic");

            let prev_balance = *locked_balance;
            let transaction_amount = rng.gen_range(MIN_TRANSACTION_AMOUNT..=MAX_TRANSACTION_AMOUNT);

            *locked_balance += transaction_amount;

            println!(
                "Processing {} of {} - balance: {} -> {}",
                if transaction_amount < 0 {
                    "withdrawal"
                } else {
                    "deposit"
                },
                format_money(transaction_amount.abs()),
                format_money(prev_balance),
                format_money(*locked_balance)
            );
        }
    })
}

/// A utility method to format money as a string.
fn format_money(balance: i64) -> String {
    let mut buf = Buffer::default();
    buf.write_formatted(&balance, &Locale::en);
    format!("${buf}")
}
