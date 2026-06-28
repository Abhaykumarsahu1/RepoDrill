mod types;
mod scanner;
mod ui;

use crate::scanner::MockScanner;
use crate::types::{ScannerEvent, SimpleFinding};
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the beautiful Terminal User Interface canvas
    let mut terminal = ui::initialize_terminal()?;

    // 2. Open our thread-safe asynchronous cross-communication pipeline (Channel)
    // We set a capacity of 100 messages to throttle data flooding safely
    let (tx, mut rx) = tokio::sync::mpsc::channel::<ScannerEvent>(100);

    // 3. Instantiate and spawn our background parsing task using Tokio
    let scanner = MockScanner::new(tx);
    tokio::spawn(async move {
        scanner.run_scan().await;
    });

    // 4. Track our dynamic application state variables
    let mut current_file = String::from("Waiting for pipeline stream...");
    let mut findings: Vec<SimpleFinding> = Vec::new();
    let mut scan_finished = false;

    // 5. The Grand Integration Frame Loop
    loop {
        // Redraw the graphics frame with the absolute latest state numbers
        terminal.draw(|f| {
            ui::draw_dashboard(f, &current_file, &findings, scan_finished);
        })?;

        // non-blocking event consumer: check if a message dropped out of our pipe
        while let Ok(event) = rx.try_recv() {
            match event {
                ScannerEvent::ProcessingFile(file_path) => {
                    current_file = file_path;
                }
                ScannerEvent::FoundIssue(finding) => {
                    findings.push(finding);
                }
                ScannerEvent::Finished => {
                    scan_finished = true;
                }
            }
        }

        // Check for quick user keyboard keystrokes to close out the window cleanly
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    // 6. Clean up the alternate layout buffers and restore the native console
    ui::restore_terminal();
    println!("✨ Thank you for using RepoDrill!");
    Ok(())
}