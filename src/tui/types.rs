use crate::tui::TerminalUserInterface;
use std::sync::Arc;

pub type NewRowCallback = Arc<dyn Fn(&mut TerminalUserInterface, Vec<String>) + Send + Sync>;
