use term;
use std::time::duration::Duration;
use std::sync::mpsc::{Sender, Receiver};
use std::old_io::timer::{sleep};
use std::sync::mpsc::channel;
use std::thread::{Thread, JoinGuard};

/// Because sometimes, you just want a global variable.
static mut show_spinner: bool = true;

pub struct Spinner {
    tx: Sender<isize>,
    guard: JoinGuard<'static ()>
}

impl Spinner {
    pub fn start() -> Spinner {
        let (tx, rx) = channel::<isize>();
        let spinner = Thread::scoped(move|| { Spinner::spin(rx) });
        Spinner{ tx: tx, guard: spinner }
    }

    pub fn stop(self) {
        let _ = self.tx.send(1);
        let _ = self.guard.join();
    }

    fn spin(rx: Receiver<isize>) {
        let spinner_chars = vec!["|", "/", "-", "\\"];
        for spin in spinner_chars.iter().cycle() {
            unsafe {
                if show_spinner {
                    say("yellow", *spin);
                }
            }
            let r = rx.try_recv();
            match r {
                Ok(_) => {
                    unsafe {
                        if show_spinner {
                            say("white", "\x08 \x08");
                        }
                    }
                    break;
                },
                Err(_) => {
                    sleep(Duration::milliseconds(100i64));
                    unsafe {
                        if show_spinner {
                            say("white", "\x08");
                        }
                    }
                    continue;
                }
            }
        }
    }
}

pub fn turn_off_spinner() {
    unsafe {
        show_spinner = false;
    }
}

fn say_term(mut t: Box<term::StdTerminal>, color: &str, to_say: &str) {
    let color_const = match color {
        "green" => term::color::BRIGHT_GREEN,
        "yellow" => term::color::BRIGHT_YELLOW,
        "red" => term::color::BRIGHT_RED,
        "magenta" => term::color::BRIGHT_MAGENTA,
        "white" => term::color::WHITE,
        _ => term::color::WHITE
    };
    t.fg(color_const).unwrap();
    t.write_all(to_say.as_bytes()).unwrap();
    t.reset().unwrap();
}

pub fn say(color: &str, to_say: &str) {
    match term::stdout() {
        Some(t) => say_term(t, color, to_say),
        None => print!("{}", to_say)
    }
}

pub fn sayln(color: &str, to_say: &str) {
    say(color, to_say);
    say(color, "\n");
}
