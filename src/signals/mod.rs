use crate::tui::tui;
use crate::utils::exit::{disable_terminal_raw_mode, exit, leave_alternate_terminal_screen_mode};
use crate::logger::logger;
use crate::utils::startup::{enable_terminal_alternate_screen_mode, enable_terminal_raw_mode};
use signal_hook::consts::{SIGINT, SIGTERM, SIGTSTP, SIGCONT};
use signal_hook::iterator::Signals;
use std::thread;

fn suspend() {
    logger().log("Suspending...");
    tui().set_is_active(false);

    leave_alternate_terminal_screen_mode();
    disable_terminal_raw_mode();

    unsafe { libc::raise(libc::SIGSTOP); }
}

fn resume() {
    logger().log("Resuming...");

    enable_terminal_alternate_screen_mode();
    enable_terminal_raw_mode();

    let mut ui = tui();
    ui.set_is_active(true);
    ui.rerender(None);
}

pub struct SignalEvent {
    signal: i32,
    on_trigger: Box<dyn Fn()>,
}

fn get_handled_events() -> Vec<SignalEvent> {
    vec![
        SignalEvent {
            signal: SIGINT,
            on_trigger: Box::new(exit),
        },
        SignalEvent {
            signal: SIGTERM,
            on_trigger: Box::new(exit),
        },
        SignalEvent {
            signal: SIGTSTP,
            on_trigger: Box::new(suspend),
        },
        SignalEvent {
            signal: SIGCONT,
            on_trigger: Box::new(resume),
        },
    ]
}

fn get_handled_event_keys() -> Vec<i32> {
    let handled_events = get_handled_events();
    let mut handled_event_keys = vec![];

    for event in handled_events {
        handled_event_keys.push(event.signal);
    }

    handled_event_keys
}

pub struct SignalsHandler {
    pub signals: Signals,
}

impl Default for SignalsHandler {
    fn default() -> Self {
        SignalsHandler::new()
    }
}

impl SignalsHandler {
    pub fn new() -> Self {
        Self {
            signals: Signals::new(get_handled_event_keys()).unwrap(),
        }
    }

    fn handle_events(&mut self) {
        for signal in self.signals.forever() {
            for handler in get_handled_events() {
                if signal == handler.signal {
                    (handler.on_trigger)();
                }
            }
        }
    }

    pub fn run(mut self) {
        thread::spawn(move || self.handle_events());
    }
}
