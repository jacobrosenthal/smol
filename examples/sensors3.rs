use anyhow::Result;
use futures::future::try_join3;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

//why not just block background thread I guess.. no benefit
fn main() {
    //create a thread
    thread::spawn(|| smol::run(async { try_join3(poll(), poll2(), render()).await }));

    //block this thread waiting on ctrlc
    smol::run(async {
        let term = Arc::new(AtomicBool::new(false));
        //unix only, but im sure i could find something else
        signal_hook::flag::register(signal_hook::SIGTERM, Arc::clone(&term)).unwrap();
        while !term.load(Ordering::Relaxed) {}
    })
}

async fn poll() -> Result<()> {
    loop {
        println!("reading");
        async_std::task::sleep(Duration::from_millis(200)).await;
    }
}

async fn poll2() -> Result<()> {
    loop {
        println!("reading2");
        async_std::task::sleep(Duration::from_millis(400)).await;
    }
}

async fn render() -> Result<()> {
    loop {
        println!("rendering");
        async_std::task::sleep(Duration::from_millis(500)).await;
    }
}
