use colored::*;
use std::io::{self, Write};

#[allow(dead_code)]
pub struct VisualIndicator {
    message: String,
    active: bool,
}

#[allow(dead_code)]
impl VisualIndicator {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
            active: false,
        }
    }
    
    pub fn start(&mut self) {
        self.active = true;
        print!("🤖 {} ", self.message.bright_cyan());
        io::stdout().flush().unwrap();
    }
    
    pub async fn show_spinner(&self) {
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        for frame in frames.iter().cycle().take(10) {
            print!("\r🤖 {}", frame);
            io::stdout().flush().unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
        print!("\r");
        io::stdout().flush().unwrap();
    }

    pub async fn show_progress(&self) {
        if !self.active { return; }
        
        let frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        for frame in frames.iter().cycle().take(10) {
            print!("\r🤖 {} {}", self.message.bright_cyan(), frame.bright_blue());
            io::stdout().flush().unwrap();
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
    
    pub fn complete(&mut self, _success: bool) {
        self.active = false;
        // Remove completion message - just clear the line
        print!("\r");
        io::stdout().flush().unwrap();
    }
    
    pub fn stream_char(&self, ch: char) {
        if self.active {
            print!("{}", ch);
            io::stdout().flush().unwrap();
        }
    }
}

pub fn start_streaming_indicator() -> VisualIndicator {
    let mut indicator = VisualIndicator::new("Streaming response");
    indicator.start();
    indicator
}
