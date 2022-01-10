mod java;
mod util;

use std::{error::Error, io};
use termion::{
    event::Key, input::MouseTerminal,
    raw::IntoRawMode, screen::AlternateScreen,
};
use tui::{
    backend::TermionBackend,
    layout::{
        Alignment, Constraint, Corner, Direction,
        Layout,
    },
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, List, ListItem},
    Terminal,
};
use util::{
    event::{Event, Events},
    StatefulList,
};

/// This struct holds the current state of the app. In particular, it has the `items` field which is a wrapper
/// around `ListState`. Keeping track of the items state let us render the associated widget with its state
/// and have access to features such as natural scrolling.
///
/// Check the event handling at the bottom to see how to change the state on incoming events.
/// Check the drawing logic for items on how to specify the highlighting style for selected items.
struct App<'a> {
    //TODO: Replace it with JmapLine
    items: StatefulList<(&'a str, usize)>,
}

impl<'a> App<'a> {
    fn new() -> App<'a> {
        App {
            items: StatefulList::with_items(
                vec![
                    ("Item0", 1),
                    ("Item1", 2),
                    ("Item2", 1),
                    ("Item3", 3),
                    ("Item4", 1),
                    ("Item5", 4),
                    ("Item6", 1),
                    ("Item7", 3),
                    ("Item8", 1),
                    ("Item9", 6),
                    ("Item10", 1),
                    ("Item11", 3),
                    ("Item12", 1),
                    ("Item13", 2),
                    ("Item14", 1),
                    ("Item15", 1),
                    ("Item16", 4),
                    ("Item17", 1),
                    ("Item18", 5),
                    ("Item19", 4),
                    ("Item20", 1),
                    ("Item21", 2),
                    ("Item22", 1),
                    ("Item23", 3),
                    ("Item24", 1),
                ],
            ),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Terminal initialization
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let events = Events::new();

    // Create a new app with some example state
    let mut app = App::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(100)].as_ref())
                .split(f.size());

            // Iterate through all elements in the `items` app and append some debug text to it.
            let items: Vec<ListItem> = app
                .items
                .items
                .iter()
                .map(|i| {
                    let mut lines = vec![Spans::from(i.0)];
                    for _ in 0..i.1 {
                        lines.push(Spans::from(Span::styled(
                            "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
                            Style::default().add_modifier(Modifier::ITALIC),
                        )));
                    }
                    return ListItem::new(lines)
                        .style(Style::default()
                               .fg(Color::White)
                               .bg(Color::Black)
                        )
                })
                .collect();

            // Create a List from all list items and highlight the currently selected one
            let items = List::new(items)
                .block(
                    Block::default()
                    .borders(Borders::ALL)
                    .title("Jmag")
                    .title_alignment(Alignment::Center)
                )
                .highlight_style(
                    Style::default()
                        .bg(Color::LightGreen)
                        .add_modifier(Modifier::BOLD),
                )
                .highlight_symbol(">> ");

            // We can now render the item list
            f.render_stateful_widget(items, chunks[0], &mut app.items.state);

        })?;

        // This is a simple example on how to handle events
        // 1. This breaks the loop and exits the program on `q` button press.
        // 2. The `up`/`down` keys change the currently selected item in the App's `items` list.
        // 3. `left` unselects the current item.
        match events.next()? {
            Event::Input(input) => match input {
                Key::Char('q') => {
                    break;
                }
                Key::Left => {
                    app.items.unselect();
                }
                Key::Char('j') => {
                    app.items.next();
                }
                Key::Char('k') => {
                    app.items.previous();
                }
                _ => {}
            },
            _ => {}
        }
    }

    Ok(())
}
