use futures::future;
use futures::future::FutureExt;
use futures::stream::{self, StreamExt};
use std::time::Duration;

use futures::channel::oneshot;
use futures::stream::{self, StreamExt};

fn main() {
    smol::run(async {
        let mut x = 0;

        {
            let fut = stream::repeat(1).take(3).for_each(|item| {
                x += item;
                future::ready(())
            });
            fut.await;
        }

        let (tx1, rx1) = oneshot::channel();
        let (tx2, rx2) = oneshot::channel();
        let (tx3, rx3) = oneshot::channel();

        let fut = stream::iter(vec![rx1, rx2, rx3]).for_each_concurrent(
            /* limit */ 1,
            |rx| async move {
                rx.await.unwrap();
            },
        );

        //superflous really since we dont give it other threads to steal on and we dont care...
        // smol::Task::local(async {
        let (s, ctrl_c) = piper::chan(1);
        ctrlc::set_handler(move || {
            let _ = s.send(()).now_or_never();
        })
        .unwrap();

        //the only thing that CAN return is ctrlc, everthing else loops
        futures::select! {
            _ = ctrl_c.recv().fuse() => (),
            _ = poll().fuse() => (),
            _ = poll2().fuse() => (),
            _ = render().fuse() => ()
        };
        // })
        // .await;
    });
}

async fn poll() {
    loop {
        println!("reading");
        async_std::task::sleep(Duration::from_millis(200)).await;
    }
}

async fn poll2() {
    loop {
        println!("reading2");
        async_std::task::sleep(Duration::from_millis(400)).await;
    }
}

async fn render() {
    loop {
        println!("render");
        async_std::task::sleep(Duration::from_millis(400)).await;
    }
}
