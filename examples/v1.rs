use std::future::Future;
use std::task::{Context, Poll};
use std::thread;
use std::time::Duration;

use futures::channel::oneshot;

/// Runs a future to completion on the current thread.
fn block_on<F: Future>(future: F) -> F::Output {
    // Pin the future on the stack.
    pin_utils::pin_mut!(future);

    // Create a waker that unparks this thread.
    let thread = thread::current();
    let waker = async_task::waker_fn(move || thread.unpark());

    // Create the task context.
    let cx = &mut Context::from_waker(&waker);

    // Keep polling the future until completion.
    loop {
        match future.as_mut().poll(cx) {
            Poll::Ready(output) => return output,
            Poll::Pending => thread::park(),
        }
    }
}

fn main() {
    let (s, r) = oneshot::channel();

    // Spawn a thread that will send a message through the channel.
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(1));
        s.send("Hello, world!").unwrap();
    });

    // Block until the message is received.
    let msg = block_on(async {
        println!("Awaiting...");
        r.await.unwrap()
    });

    println!("{}", msg);
}
