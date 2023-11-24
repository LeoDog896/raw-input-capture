use crossterm::execute;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::{self, stdout, Read};
use std::num::NonZeroU8;
use std::sync::mpsc::{self, Receiver};
use std::{str, thread};

pub struct TTYTerminal {
    channel: Receiver<u8>,
}

impl Default for TTYTerminal {
    fn default() -> Self {
        Self::new()
    }
}

impl TTYTerminal {
    #[must_use]
    pub fn new() -> Self {
        enable_raw_mode().unwrap();

        let stdin_channel = spawn_stdin_channel();

        Self {
            channel: stdin_channel,
        }
    }

    fn get_input(&mut self) -> Option<NonZeroU8> {
        self.channel.try_recv().ok().and_then(NonZeroU8::new)
    }
}

impl Drop for TTYTerminal {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
    }
}

/// Spawn a thread to read stdin and send it to a channel.
/// Since stdin().bytes() is blocking, we need to spawn a thread to read it.
/// This allows us to poll the channel instead of blocking on stdin.
fn spawn_stdin_channel() -> Receiver<u8> {
    let (tx, rx) = mpsc::channel::<u8>();
    thread::spawn(move || loop {
        let ch = io::stdin()
            .bytes()
            .next()
            .and_then(Result::ok)
            .unwrap();
        tx.send(ch).unwrap();
    });
    rx
}

fn main() {
    let mut terminal = TTYTerminal::new();

    loop {
        if let Some(key) = terminal.get_input() {
            execute!(
                stdout(),
                Print(format!(
                    "\n{} - {}",
                    str::from_utf8(&[key.get()]).unwrap(),
                    key.get()
                ))
            )
            .unwrap();
        }
    }
}
