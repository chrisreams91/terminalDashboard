mod app;
mod requests;
mod ui;
mod util;

use std::io::{stdin, stdout};
use std::sync::mpsc;
use std::thread;
use tokio::time::Duration;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::Terminal;

use hyper::Client;
use hyper_tls::HttpsConnector;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[tokio::main]
async fn main() -> Result<()> {
    let stdout = stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear().unwrap();

    let (tx, rx) = mpsc::channel();
    let mut should_exit = false;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let mut app = app::App::new();

    println!("fetching data...");
    app.fetch_all_data(&client).await?;

    // separate thread for listening to key events then emits back to main thread
    thread::spawn(move || {
        let stdin = stdin();
        let tx = tx.clone();

        for key in stdin.keys() {
            match key.unwrap() {
                Key::Char('q') => tx.send(Key::Char('q')).unwrap(),
                Key::Char('Q') => tx.send(Key::Char('Q')).unwrap(),
                Key::Ctrl('c') => tx.send(Key::Ctrl('c')).unwrap(),
                Key::Char('r') => tx.send(Key::Char('r')).unwrap(),
                Key::Char('p') => tx.send(Key::Char('p')).unwrap(),
                Key::Up => tx.send(Key::Up).unwrap(),
                Key::Down => tx.send(Key::Down).unwrap(),
                Key::Char('\n') => tx.send(Key::Char('\n')).unwrap(),
                _ => {}
            }
        }
    });

    // draws the UI in loop to handle re-rendering
    loop {
        terminal.draw(|f| {
            ui::draw_ui(f, &mut app).unwrap();
        })?;

        match rx.try_recv() {
            Ok(Key::Char('q')) => should_exit = true,
            Ok(Key::Char('Q')) => should_exit = true,
            Ok(Key::Ctrl('c')) => should_exit = true,
            Ok(Key::Char('p')) => app.party_mode = !app.party_mode,
            Ok(Key::Char('r')) => {
                app.fetch_all_data(&client).await?;
            }
            Ok(Key::Up) => app.select_previous_gitlab_application(),
            Ok(Key::Down) => app.select_next_gitlab_application(),
            Ok(Key::Char('\n')) => {
                app.selected_application = app.selected_list_item.to_owned();
                app.fetch_gitlab_data(&client).await;
            }
            _ => {}
        }

        if should_exit == true {
            break;
        }

        // slowing down the looping
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
