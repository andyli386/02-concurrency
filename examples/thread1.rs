use std::{sync::mpsc, thread, time::Duration};

use anyhow::{anyhow, Result};

#[allow(dead_code)]
#[derive(Debug)]
struct Msg {
    idx: usize,
    value: usize,
}

const NUM_PRODUCERS: usize = 4;

fn main() -> Result<()> {
    // print!("Hello World!");
    let (tx, rx) = mpsc::channel();
    for i in 0..NUM_PRODUCERS {
        let tx = tx.clone();
        thread::spawn(move || producer(i, tx));
    }
    drop(tx);

    let consumer = thread::spawn(move || {
        for msg in rx {
            println!("consume {:?}", msg);
        }
    });

    let _ = consumer
        .join()
        .map_err(|e| anyhow!("Thread join error {:?}", e));

    println!("Bye");
    Ok(())
}

fn producer(idx: usize, tx: mpsc::Sender<Msg>) -> Result<()> {
    loop {
        let value = rand::random::<usize>();
        tx.send(Msg { idx, value })?;
        let sleep_time = rand::random::<u8>() as u64 * 10;
        thread::sleep(Duration::from_millis(sleep_time));

        let random_exit = rand::random::<u8>();
        if random_exit % 10 == 0 {
            println!("exit {}", idx);
            break;
        }
    }
    Ok(())
}

#[allow(dead_code)]
impl Msg {
    fn new(idx: usize, value: usize) -> Self {
        Self { idx, value }
    }
}
