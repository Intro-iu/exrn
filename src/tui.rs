use crate::planner::RenameAction;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Cell, Row, Table, TableState,
    },
    Frame, Terminal, TerminalOptions, Viewport,
};
use std::error::Error;
use std::io;

pub struct App {
    pub actions: Vec<RenameAction>,
    pub selected: Vec<bool>, // true if action is selected for execution
    pub state: TableState,
    pub show_help: bool,
}

impl App {
    pub fn new(actions: Vec<RenameAction>) -> App {
        let count = actions.len();
        App {
            actions,
            selected: vec![true; count], // Default all selected
            state: TableState::default().with_selected(0),
            show_help: true,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.actions.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.actions.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn toggle_selection(&mut self) {
        if let Some(i) = self.state.selected() {
            self.selected[i] = !self.selected[i];
        }
    }
}

pub fn run_tui(actions: Vec<RenameAction>) -> Result<Option<Vec<RenameAction>>, Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create App state
    let mut app = App::new(actions);

    // Run loop
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Ok(confirmed) = res {
        if confirmed {
            // Filter actions
            let final_actions: Vec<RenameAction> = app
                .actions
                .into_iter()
                .zip(app.selected.into_iter())
                .filter(|(_, selected)| *selected)
                .map(|(action, _)| action)
                .collect();
            Ok(Some(final_actions))
        } else {
            Ok(None) // Cancelled
        }
    } else {
        Err(res.err().unwrap())
    }
}

pub fn print_preview(actions: Vec<RenameAction>) -> Result<(), Box<dyn Error>> {
    let mut app = App::new(actions);
    app.show_help = false;
    app.state.select(None); // Disable cursor highlight

    // Calculate layout parameters
    let term_size = crossterm::terminal::size()?;
    let width = term_size.0;
    
    // Calculate total height needed based on wrapping
    // Table width = terminal width - 2 (borders)
    // Column widths: Matches constraints in render_table
    // 1: 10 chars
    // 2: 45% (approx)
    // 3: 45% (approx)
    
    let table_width = if width > 2 { width - 2 } else { width };
    let col2_width = (table_width as f64 * 0.45) as usize;
    // Ensure at least 1 char width
    let wrap_width = if col2_width < 1 { 1 } else { col2_width };

    let mut total_height = 4; // Header(2) + Borders(2)
    for action in &app.actions {
        let src = action.source.file_name().unwrap_or_default().to_string_lossy();
        let target = action.target.file_name().unwrap_or_default().to_string_lossy();
        
        let src_lines = (src.chars().count() + wrap_width - 1) / wrap_width;
        let target_lines = (target.chars().count() + wrap_width - 1) / wrap_width;
        let row_height = std::cmp::max(src_lines, target_lines).max(1);
        total_height += row_height as u16;
    }

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::with_options(
        backend,
        TerminalOptions {
            viewport: Viewport::Inline(total_height),
        },
    )?;

    terminal.draw(|f| ui(f, &mut app))?;
    Ok(())
}

fn wrap_text(text: &str, width: usize) -> String {
    if width == 0 { return text.to_string(); }
    let chars: Vec<char> = text.chars().collect();
    if chars.is_empty() { return String::new(); }
    
    chars.chunks(width)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>, app: &mut App) -> Result<bool, Box<dyn Error>> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match key.code {
                KeyCode::Char('q') | KeyCode::Esc => return Ok(false),
                KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => return Ok(false),
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Char(' ') => app.toggle_selection(),
                KeyCode::Enter => return Ok(true),
                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App) {
    render_table(f, app, f.area());
}

fn render_table(f: &mut Frame, app: &mut App, area: Rect) {
    let header_cells = ["Selected", "Source", "Target"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::White)));
    let header = Row::new(header_cells)
        .style(Style::default().add_modifier(Modifier::BOLD))
        .height(1)
        .bottom_margin(1);

    // Calculate wrapping for rendering
    let table_width = if area.width > 2 { area.width - 2 } else { area.width };
    let col_width = (table_width as f64 * 0.45) as usize;
    let wrap_width = if col_width < 1 { 1 } else { col_width };

    let rows = app.actions.iter().enumerate().map(|(i, action)| {
        let is_selected = app.selected[i];
        let has_cursor = app.state.selected().map_or(false, |s| s == i);

        let checkbox = if is_selected { "[x]" } else { "[ ]" };
        let checkbox_style = if has_cursor {
            Style::default().fg(Color::White)
        } else {
            if is_selected { Style::default().fg(Color::White) } else { Style::default().fg(Color::DarkGray) }
        };

        let src = action.source.file_name().unwrap_or_default().to_string_lossy();
        let target = action.target.file_name().unwrap_or_default().to_string_lossy();
        
        let src_wrapped = wrap_text(&src, wrap_width);
        let target_wrapped = wrap_text(&target, wrap_width);
        
        // Calculate height

        let src_len = src.chars().count();
        let target_len = target.chars().count();
        let s_lines = if src_len == 0 { 1 } else { (src_len + wrap_width - 1) / wrap_width };
        let t_lines = if target_len == 0 { 1 } else { (target_len + wrap_width - 1) / wrap_width };
        let row_height = std::cmp::max(s_lines, t_lines).max(1) as u16;

        // Colors
        let (src_style, target_style) = if has_cursor || is_selected {
            (Style::default().fg(Color::White), Style::default().fg(Color::White))
        } else {
            (Style::default().fg(Color::DarkGray), Style::default().fg(Color::DarkGray))
        };

        let cells = vec![
            Cell::from(checkbox).style(checkbox_style),
            Cell::from(src_wrapped).style(src_style),
            Cell::from(target_wrapped).style(target_style),
        ];
        Row::new(cells).height(row_height).bottom_margin(0)
    });

    let mut table = Table::new(
        rows,
        [
            Constraint::Length(10),
            Constraint::Percentage(45),
            Constraint::Percentage(45),
        ],
    )
    .header(header);

    let mut block = Block::default()
            .borders(Borders::ALL)
            .title(" exrn - Review Changes ")
            .border_style(Style::default().fg(Color::Rgb(85, 108, 255)));

    if app.show_help {
        let help_text = Line::from(vec![
            Span::raw(" Controls: "),
            Span::styled("↑/↓", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Select | "),
            Span::styled("Space", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Toggle | "),
            Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Confirm | "),
            Span::styled("q/Esc", Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" Quit "),
        ]);
        block = block.title_bottom(help_text);
    }
    table = table.block(block);

    if app.state.selected().is_some() {
        table = table.highlight_symbol(" >> ")
             .row_highlight_style(Style::default().bg(Color::Rgb(123, 141, 255)).fg(Color::White).add_modifier(Modifier::BOLD));
    }

    f.render_stateful_widget(table, area, &mut app.state);
}
