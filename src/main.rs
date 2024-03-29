use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::process::{Command, Stdio};
use susum::app::{App, AppResult};
use susum::aws::get_instances;
use susum::event::{Event, EventHandler};
use susum::handler::handle_key_events;
use susum::ports::{discover_free_port, wait_port_freed};
use susum::tui::Tui;

#[tokio::main]
async fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(60);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    let (tx, mut rx) = tokio::sync::mpsc::channel(1);

    tokio::spawn(async move {
        let instances = get_instances().await;
        _ = tx.send(instances).await;
    });

    let port = discover_free_port().await;
    app.port = port;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        tokio::select! {
            event = tui.events.next() => {
                match event? {
                    Event::Tick => app.tick(),
                    Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
                    Event::Mouse(_) => {}
                    Event::Resize(_, _) => {}
                }
            }
            Some(loaded) = rx.recv() => {
                app.load(loaded)
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;

    // TODO: Panic if esc is used when list is filtered to none.
    let instance = match app.list_state.selected() {
        Some(i) => Some(app.filtered[i].clone()),
        None => None,
    };

    if app.start_session && instance.is_some() {
        println!(
            "Connecting to {} on port {}",
            &instance.clone().unwrap().display(),
            app.port.unwrap()
        );
        let mut child = Command::new("aws")
            .arg("ssm")
            .arg("start-session")
            .arg("--document-name")
            .arg("AWS-StartPortForwardingSession")
            .arg("--parameters")
            .arg(format!(
                r#"{{"portNumber": ["3389"], "localPortNumber": ["{}"]}}"#,
                port.expect("No ports free")
            ))
            .arg("--target")
            .arg(instance.unwrap().instance_id)
            .stdin(Stdio::inherit())
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn()?;

        let status = child.wait().expect("Failed to wait on child");

        if !wait_port_freed(app.port.unwrap()).await {
            eprintln!("Port not freed!")
        }

        println!("Process exited with status: {:?}", status);
    }

    Ok(())
}
