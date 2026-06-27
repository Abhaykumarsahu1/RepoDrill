use crate::types::SimpleFinding;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

/// Safely sets up the terminal by entering raw mode and switching to the alternate screen.
pub fn initialize_terminal() -> Result<ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>, std::io::Error> {
    use crossterm::{execute, terminal::{EnterAlternateScreen, enable_raw_mode}};
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    ratatui::Terminal::new(backend)
}

/// Safely restores the standard terminal screen when the program exits.
pub fn restore_terminal() {
    use crossterm::{execute, terminal::{LeaveAlternateScreen, disable_raw_mode}};
    let _ = disable_raw_mode();
    let _ = execute!(std::io::stdout(), LeaveAlternateScreen);
}

/// Draws our layout layout on every single frame refresh (60 FPS).
pub fn draw_dashboard(
    f: &mut Frame,
    current_file: &str,
    findings: &[SimpleFinding],
    scan_finished: bool,
) {
    // 1. Split the screen vertically into 3 sections: Header, Active Scanner, and Issues list.
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header bar
            Constraint::Length(3),  // Current File status panel
            Constraint::Min(5),     // Scrollable Alerts / Findings list
        ])
        .split(f.area()); // Updated from .size() to match the newest Ratatui API

    // 2. The Header Panel
    let status_text = if scan_finished {
        " STATUS: SCAN COMPLETE (100%) "
    } else {
        " STATUS: DRILLING CODEBASE... "
    };
    
    let header = Paragraph::new(format!(" ⚔️ REPODRILL v0.1.0 |{}", status_text))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::DarkGray)));
    f.render_widget(header, chunks[0]);

    // 3. The Current Processing File Panel
    let file_display = if scan_finished {
        format!(" Finished parsing target repository. Found {} anomalies.", findings.len())
    } else {
        format!(" 📂 Analyzing: {}", current_file)
    };

    let file_block = Paragraph::new(file_display)
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().title(" Active Worker Pipeline ").borders(Borders::ALL));
    f.render_widget(file_block, chunks[1]);

    // 4. The Security Findings Panel
    let list_items: Vec<ListItem> = findings
        .iter()
        .map(|fnd| {
            let color = if fnd.message.contains("CRITICAL") {
                Color::Red
            } else {
                Color::Magenta
            };
            ListItem::new(format!(
                " [{}] Line {}: {}",
                fnd.file_path, fnd.line_number, fnd.message
            ))
            .style(Style::default().fg(color))
        })
        .collect();

    let list_block = List::new(list_items)
        .block(Block::default().title(" Vulnerability & Debt Alerts ").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC));
    f.render_widget(list_block, chunks[2]);
}