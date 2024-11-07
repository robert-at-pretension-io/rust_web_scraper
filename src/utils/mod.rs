use std::io::{self, Write};
use std::time::Duration;
use tokio::time::sleep;

pub struct Spinner {
    frames: Vec<&'static str>,
    current: usize,
    message: String,
}

impl Spinner {
    pub fn new(message: &str) -> Self {
        Self {
            frames: vec!["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
            current: 0,
            message: message.to_string(),
        }
    }

    pub fn tick(&mut self) {
        print!("\r{} {} ", self.frames[self.current], self.message);
        io::stdout().flush().unwrap();
        self.current = (self.current + 1) % self.frames.len();
    }

    pub fn stop(&self) {
        print!("\r✓ {}\n", self.message);
        io::stdout().flush().unwrap();
    }
}

pub async fn with_spinner<F, T>(message: &str, future: F) -> T 
where
    F: std::future::Future<Output = T>,
{
    let message = message.to_string();
    let (tx, mut rx) = tokio::sync::oneshot::channel();
    
    // Spawn spinner task
    let spinner_handle = tokio::spawn(async move {
        let mut spinner = Spinner::new(&message);
        loop {
            spinner.tick();
            tokio::select! {
                _ = tokio::time::sleep(Duration::from_millis(80)) => continue,
                result = &mut rx => {
                    if result.is_ok() {
                        break;
                    }
                }
            }
        }
        print!("\r✓ {}\n", message);
        io::stdout().flush().unwrap();
    });

    // Run the actual future
    let result = future.await;
    
    // Signal spinner to stop
    let _ = tx.send(());
    let _ = spinner_handle.await;
    
    result
}
