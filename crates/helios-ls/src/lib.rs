mod connection;
mod error;
mod protocol;
mod server;
mod state;

use server::Server;
use state::State;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Establishes a connection to the client and starts the server.
pub fn start() {
    if let Err(error) = __start() {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

/// Responsible for the server's main loop logic.
fn __start() -> Result<()> {
    let (connection, threads) = connection::stdio();

    let mut state = State::new(connection.sender);
    Server::new(connection.receiver, &mut state).initialize()?.run()?;

    threads.join()?;
    log::info!("Connection to client has closed");

    Ok(())
}
