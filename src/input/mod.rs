use crate::logger::logger;
use crate::utils::exit::exit;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use once_cell::sync::OnceCell;
use std::sync::{Arc, atomic::AtomicBool, atomic::Ordering};
use std::thread;
use std::time::Duration;

fn suspend() {
    unsafe {
        libc::raise(libc::SIGTSTP);
    }
}

struct UserInputEvent {
    key: KeyCode,
    modifier: KeyModifiers,
    on_trigger: Box<dyn Fn()>,
}

fn get_handled_events() -> Vec<UserInputEvent> {
    vec![
        UserInputEvent {
            key: KeyCode::Char('c'),
            modifier: KeyModifiers::CONTROL,
            on_trigger: Box::new(|| {
                logger().log("^C");
                exit();
            }),
        },
        UserInputEvent {
            key: KeyCode::Char('q'),
            modifier: KeyModifiers::NONE,
            on_trigger: Box::new(|| {
                logger().log("q");
                exit();
            }),
        },
        UserInputEvent {
            key: KeyCode::Char('z'),
            modifier: KeyModifiers::CONTROL,
            on_trigger: Box::new(|| {
                logger().log("^Z");
                suspend();
            }),
        },
        UserInputEvent {
            key: KeyCode::Char('b'),
            modifier: KeyModifiers::NONE,
            on_trigger: Box::new(|| {
                logger().log("b");
                suspend();
            }),
        },
    ]
}

pub struct UserInputHandler {
    events_check_frequency: u64,
    is_active: AtomicBool,
}

impl Default for UserInputHandler {
    fn default() -> Self {
        UserInputHandler::new()
    }
}

impl UserInputHandler {
    pub fn new() -> Self {
        Self {
            events_check_frequency: 500,
            is_active: AtomicBool::new(true),
        }
    }

    fn handle_events(self: Arc<Self>) {
        loop {
            if !self.is_active.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_millis(self.events_check_frequency));
                continue;
            }

            if event::poll(Duration::from_millis(self.events_check_frequency)).unwrap()
                && let Event::Key(KeyEvent {
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

    pub fn run(self: Arc<Self>) {
        let cloned_reference = self.clone();
        thread::spawn(move || cloned_reference.handle_events());
    }

    pub fn set_is_active(self: Arc<Self>, new_value: bool) {
        self.is_active.store(new_value, Ordering::SeqCst);
    }
}

static USER_INPUT_HANDLER: OnceCell<Arc<UserInputHandler>> = OnceCell::new();

pub fn user_input_handler() -> Arc<UserInputHandler> {
    USER_INPUT_HANDLER
        .get_or_init(|| {
            let handler = Arc::new(UserInputHandler::new());
            handler.clone().run();
            handler
        })
        .clone()
}
