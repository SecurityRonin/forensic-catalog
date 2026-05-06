pub mod app;
pub mod dataset;
pub mod guards;
pub mod heatmap;
pub mod keys;
pub mod presets;
pub mod search;
pub mod theme;
pub mod ui;

#[cfg(test)]
mod tests {
    use super::*;

    fn make_app(dataset: usize, query: &str, preset: usize) -> app::App {
        let mut a = app::App::new();
        a.switch_dataset(dataset);
        a.search_query = query.to_string();
        a.preset_idx = preset;
        a
    }

    #[test]
    fn build_render_data_catalog_full_length() {
        let a = make_app(0, "", 0);
        let rd = build_render_data(&a);
        assert!(
            rd.list_items.len() > 100,
            "catalog must have >100 items, got {}",
            rd.list_items.len()
        );
    }

    #[test]
    fn build_render_data_windows_lolbins_dataset() {
        let a = make_app(1, "", 0);
        let rd = build_render_data(&a);
        assert!(!rd.list_items.is_empty(), "windows lolbins must be non-empty");
    }

    #[test]
    fn build_render_data_preset_windows_crit_filters() {
        let a = make_app(0, "", 1); // preset 1 = Windows CRIT
        let rd = build_render_data(&a);
        let full_count = {
            let a2 = make_app(0, "", 0);
            build_render_data(&a2).list_items.len()
        };
        assert!(
            rd.list_items.len() < full_count,
            "Windows CRIT preset must filter catalog; got {} vs full {}",
            rd.list_items.len(),
            full_count
        );
        for item in &rd.list_items {
            assert!(item.contains("Critical"), "item must be Critical: {item}");
        }
    }

    #[test]
    fn build_render_data_search_filters_catalog() {
        let a = make_app(0, "prefetch", 0);
        let rd = build_render_data(&a);
        assert!(!rd.list_items.is_empty(), "search 'prefetch' must match something");
        for item in &rd.list_items {
            assert!(
                item.to_lowercase().contains("prefetch"),
                "filtered item must contain query: {item}"
            );
        }
    }

    #[test]
    fn build_render_data_empty_query_returns_all() {
        let a = make_app(0, "", 0);
        let rd = build_render_data(&a);
        let expected = forensicnomicon::catalog::CATALOG.list().len();
        assert_eq!(rd.list_items.len(), expected);
    }

    #[test]
    fn build_render_data_detail_shows_selected_artifact() {
        let mut a = make_app(0, "prefetch_file", 0);
        a.selected = 0; // first search result
        let rd = build_render_data(&a);
        let combined = rd.detail_lines.join("\n");
        assert!(
            combined.contains("prefetch") || combined.contains("Prefetch"),
            "detail must mention selected artifact; got: {combined}"
        );
    }

    #[test]
    fn load_theme_returns_default_on_missing_file() {
        let t = load_theme();
        assert_ne!(t.crit_fg, ratatui::style::Color::Reset);
    }
}

use crossterm::{
    event::{self, Event},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

/// Launch the interactive TUI navigator.
///
/// Called when `4n6query` is invoked with no arguments on a TTY.
/// Returns 0 on clean exit, 1 on error.
pub fn run() -> i32 {
    if let Err(e) = run_inner() {
        eprintln!("tui error: {e}");
        1
    } else {
        0
    }
}

fn run_inner() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = app::App::new();
    let theme = theme::ALL_THEMES[0]; // default theme

    // Placeholder data until full data integration
    let list_items: Vec<String> = forensicnomicon::catalog::CATALOG
        .list()
        .iter()
        .take(100)
        .map(|d| format!("{:<30} {:?}", d.id, d.triage_priority))
        .collect();
    let detail_lines: Vec<String> = vec!["Select an item to see details.".into()];

    loop {
        app.tick_flash();

        terminal.draw(|f| {
            ui::draw(f, &app, theme, &list_items, &detail_lines);
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if keys::handle_key(&mut app, key, list_items.len()) {
                    break;
                }
            }
        }
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}
