use std::io;
use std::path::PathBuf;
use tui::Terminal;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders, List, ListItem};

mod explorer;

fn main() -> Result<(), io::Error> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let my_dir = explorer::Explorer {
        path: PathBuf::from(".."),
    };
    let entries = my_dir.list_dict();
    let items: Vec<ListItem> = entries.iter().map(|n| ListItem::new(n.as_str())).collect();

    let list = List::new(items).block(Block::default().title("Files").borders(Borders::ALL));

    terminal.draw(|f| {
        let size = f.size();
        f.render_widget(list, size);
    })?;

    Ok(())
}
