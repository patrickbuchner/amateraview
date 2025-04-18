use eyre::{Context, Result};
use iced::Theme;
use iced::widget::{Column, button, column, text};
use tracing::{info, instrument};

fn main() -> Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .pretty()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(true)
        // Build the subscriber
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .wrap_err("Failed to initialize the subscriber.")?;

    iced::application("A counter", update, view)
        .theme(|_| Theme::Dark)
        .centered()
        .run()
        .wrap_err("Failed to run the application.")
}

#[derive(Debug, Clone)]
enum Message {
    Increment,
}
#[instrument]
fn update(value: &mut u64, message: Message) {
    info!("{:?}", message);
    match message {
        Message::Increment => *value += 1,
    }
    info!("Leaving");
}

fn view(value: &u64) -> Column<Message> {
    column![text(value), button("+").on_press(Message::Increment),]
}
