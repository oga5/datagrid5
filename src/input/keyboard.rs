/// Keyboard event handler for grid navigation
pub struct KeyboardHandler {
    // Currently not storing state, but could be extended
}

impl KeyboardHandler {
    pub fn new() -> Self {
        Self {}
    }

    /// Handle keyboard event and return navigation command
    pub fn handle_key(&self, key: &str) -> Option<NavigationCommand> {
        match key {
            // Arrow keys
            "ArrowUp" => Some(NavigationCommand::MoveUp),
            "ArrowDown" => Some(NavigationCommand::MoveDown),
            "ArrowLeft" => Some(NavigationCommand::MoveLeft),
            "ArrowRight" => Some(NavigationCommand::MoveRight),

            // Page navigation
            "PageUp" => Some(NavigationCommand::PageUp),
            "PageDown" => Some(NavigationCommand::PageDown),

            // Home/End
            "Home" => Some(NavigationCommand::Home),
            "End" => Some(NavigationCommand::End),

            // Enter key for edit mode (future)
            "Enter" => Some(NavigationCommand::Enter),

            // Escape key
            "Escape" => Some(NavigationCommand::Escape),

            // Tab key
            "Tab" => Some(NavigationCommand::Tab),

            // Delete key
            "Delete" => Some(NavigationCommand::Delete),

            _ => None,
        }
    }

    /// Handle keyboard event with modifier keys
    pub fn handle_key_with_modifiers(&self, key: &str, ctrl: bool) -> Option<NavigationCommand> {
        if ctrl {
            match key {
                "Home" => Some(NavigationCommand::DocumentStart),
                "End" => Some(NavigationCommand::DocumentEnd),
                _ => self.handle_key(key),
            }
        } else {
            self.handle_key(key)
        }
    }
}

impl Default for KeyboardHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation commands from keyboard input
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NavigationCommand {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
    Home,
    End,
    Enter,
    Escape,
    Tab,
    Delete,
    DocumentStart,
    DocumentEnd,
}
