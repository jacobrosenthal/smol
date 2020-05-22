use anyhow::Result;
use futures::future::try_join;
use smol::Task;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

//does it buy anything over select?
fn main() {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .unwrap();

    smol::run(async {
        //spin off the sensor polling
        Task::spawn(async { drop(try_join(poll(), poll2()).await) }).detach();

        while running.load(Ordering::SeqCst) {
            println!("rendering");
            async_std::task::sleep(Duration::from_millis(500)).await;
        }
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
