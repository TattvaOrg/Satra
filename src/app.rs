use crate::session::Session;
use crossterm::event::{self, KeyCode, KeyEventKind};
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum Focus {
    Input,
    List,
}

pub struct App {
    pub input: String,
    pub focus: Focus,
    pub sessions: Vec<Session>,
    pub selected_index: usize,
    pub show_detail: bool,
    pub should_quit: bool,
    next_id: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            focus: Focus::Input,
            sessions: Vec::new(),
            selected_index: 0,
            show_detail: false,
            should_quit: false,
            next_id: 1,
        }
    }

    pub fn tick(&mut self) {
        let mut to_remove = Vec::new();
        for (i, session) in self.sessions.iter_mut().enumerate() {
            session.update_status();
            if let Some(exited_at) = session.exited_at {
                if exited_at.elapsed() > Duration::from_secs(5 * 60) {
                    to_remove.push(i);
                }
            }
        }

        for i in to_remove.into_iter().rev() {
            self.sessions.remove(i);
        }

        if self.selected_index >= self.sessions.len() && !self.sessions.is_empty() {
            self.selected_index = self.sessions.len() - 1;
        }
    }

    pub fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        if key.code == KeyCode::Char('c') && key.modifiers.contains(event::KeyModifiers::CONTROL) {
            self.should_quit = true;
            return;
        }

        if self.show_detail {
            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => self.show_detail = false,
                KeyCode::Char('k') => {
                    if let Some(session) = self.sessions.get_mut(self.selected_index) {
                        let _ = session.kill();
                    }
                }
                KeyCode::Char('r') => {
                    if let Some(session) = self.sessions.get(self.selected_index) {
                        if let Ok(new_session) = Session::new(self.next_id, &session.command) {
                            self.next_id += 1;
                            self.sessions.push(new_session);
                        }
                    }
                    self.show_detail = false;
                }
                KeyCode::Char('d') => {
                    if self.selected_index < self.sessions.len() {
                        let mut session = self.sessions.remove(self.selected_index);
                        let _ = session.kill();
                        if self.selected_index >= self.sessions.len() && !self.sessions.is_empty() {
                            self.selected_index = self.sessions.len() - 1;
                        }
                    }
                    self.show_detail = false;
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Tab => {
                self.focus = match self.focus {
                    Focus::Input => Focus::List,
                    Focus::List => Focus::Input,
                };
            }
            KeyCode::Esc => {
                if matches!(self.focus, Focus::List) {
                    self.focus = Focus::Input;
                }
            }
            KeyCode::Down if matches!(self.focus, Focus::List) => {
                if !self.sessions.is_empty() && self.selected_index < self.sessions.len() - 1 {
                    self.selected_index += 1;
                }
            }
            KeyCode::Up if matches!(self.focus, Focus::List) => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
            }
            KeyCode::Enter => match self.focus {
                Focus::Input => {
                    if !self.input.is_empty() {
                        if let Ok(session) = Session::new(self.next_id, &self.input) {
                            self.sessions.push(session);
                            self.next_id += 1;
                            self.input.clear();
                        }
                    }
                }
                Focus::List => {
                    if !self.sessions.is_empty() {
                        self.show_detail = true;
                    }
                }
            },
            KeyCode::Char(c) if matches!(self.focus, Focus::Input) => {
                self.input.push(c);
            }
            KeyCode::Backspace if matches!(self.focus, Focus::Input) => {
                self.input.pop();
            }
            _ => {}
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyEvent, KeyModifiers};

    #[test]
    fn test_app_init() {
        let app = App::new();
        assert_eq!(app.selected_index, 0);
        assert_eq!(app.focus, Focus::Input);
        assert!(!app.show_detail);
        assert!(!app.should_quit);
    }

    #[test]
    fn test_focus_toggle() {
        let mut app = App::new();
        let tab_key = KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE);
        app.handle_key(tab_key);
        assert_eq!(app.focus, Focus::List);
        app.handle_key(tab_key);
        assert_eq!(app.focus, Focus::Input);
    }

    #[test]
    fn test_esc_returns_to_input() {
        let mut app = App::new();
        app.focus = Focus::List;
        let esc_key = KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE);
        app.handle_key(esc_key);
        assert_eq!(app.focus, Focus::Input);
    }

    #[test]
    fn test_ctrl_c_quits() {
        let mut app = App::new();
        let ctrl_c = KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL);
        app.handle_key(ctrl_c);
        assert!(app.should_quit);
    }
}
