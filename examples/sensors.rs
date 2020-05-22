use futures::future::FutureExt;
use std::time::Duration;

fn main() {
    smol::run(async {
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
