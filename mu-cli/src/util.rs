use std::path::Path;

use colored::Colorize;
use futures::Stream;
use notify::{Event, RecursiveMode, Watcher};
use terminal_size::{terminal_size, Width};
use tokio::sync::mpsc::{self, UnboundedReceiver};

pub fn print_full_line(message: &str) {
    let width = match terminal_size() {
        Some((Width(w), _)) => w as usize,
        None => 80, // Fallback width if terminal size can't be detected
    };
    let message: String = format!("[μ]: {} ", message);
    let padded_message = format!("{:<width$}", message, width = width)
        .black()
        .on_green()
        .bold();

    println!("{}", padded_message);
}

pub struct MyWatcher {
    _watcher: notify::RecommendedWatcher,
    event_rx: UnboundedReceiver<()>,
    enabled: bool,
    waker: Option<std::task::Waker>,
}

impl MyWatcher {
    pub fn new(path: &str) -> Self {
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| match res {
            Ok(event) => {
                if event.kind.is_create() || event.kind.is_modify() || event.kind.is_remove() {
                    event_tx.send(()).unwrap();
                }
            }
            Err(e) => {
                println!("watch error: {:?}", e);
            }
        })
        .unwrap();
        watcher
            .watch(Path::new(path), RecursiveMode::Recursive)
            .unwrap();

        Self {
            _watcher: watcher,
            event_rx,
            enabled: false,
            waker: None,
        }
    }

    pub fn enable(&mut self) {
        while self.event_rx.try_recv().is_ok() {} // empty the queue
        self.enabled = true;
        if let Some(waker) = self.waker.as_ref() {
            waker.wake_by_ref();
        }
    }
}

impl Stream for MyWatcher {
    type Item = ();

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context,
    ) -> std::task::Poll<Option<Self::Item>> {
        if self.enabled {
            let this = self.get_mut();
            let result = this.event_rx.poll_recv(cx);
            if result.is_ready() {
                this.enabled = false;
            }
            result
        } else {
            self.get_mut().waker = Some(cx.waker().clone());
            std::task::Poll::Pending
        }
    }
}
