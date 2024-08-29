/*
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    handle.write_all(b"hello world")?;

    Ok(())
}
*/
/*
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
}
*/
use std::thread;

fn main() {
    let mut n = String::new();

    std::io::stdin()
        .read_line(&mut n)
        .expect("Err n");

    let n: i64 = n
        .trim()
        .parse()
        .expect("Err parse");

    let mut handles = vec![];

    for num_1_to_n in 0..n {
        let handle = thread::spawn(move || {
            
            println!("{}^2 = {}", num_1_to_n, num_1_to_n * num_1_to_n);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}