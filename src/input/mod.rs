use crate::logger::logger;
use crate::utils::exit::exit;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use std::thread;
use std::time::Duration;

pub struct UserInputEvent {
    key: KeyCode,
    modifier: KeyModifiers,
    on_trigger: Box<dyn Fn()>,
}

pub fn get_handled_events() -> Vec<UserInputEvent> {
    let mut handled_events: Vec<UserInputEvent> = vec![];

    handled_events.push(UserInputEvent {
        key: KeyCode::Char('c'),
        modifier: KeyModifiers::CONTROL,
        on_trigger: Box::new(|| {
            logger().log("^C");
            exit();
        }),
    });

    handled_events
}

pub struct UserInputHandler {
    events_check_frequency: u64,
}

impl Default for UserInputHandler {
    fn default() -> Self {
        UserInputHandler::new()
    }
}

impl UserInputHandler {
    pub fn new() -> Self {
        UserInputHandler {
            events_check_frequency: 500,
        }
    }

    fn handle_events(&self) {
        loop {
            if event::poll(Duration::from_millis(self.events_check_frequency)).unwrap() {
                if let Event::Key(KeyEvent {
                    code, modifiers, ..
                }) = event::read().unwrap()
                {
                    let event_handlers = get_handled_events();
                    for handler in event_handlers {
                        if handler.key == code && handler.modifier == modifiers {
                            (handler.on_trigger)();
                        }
                    }
                }
            }
        }
    }

    pub fn run(self) {
        thread::spawn(move || self.handle_events());
    }
}
