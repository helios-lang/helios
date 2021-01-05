use crate::protocol::Message;
use flume::{Receiver, Sender};
use std::io;
use std::thread;

/// A structure that holds the sender and receiver of parsed [`Message`]s.
pub struct Connection {
    /// Sender channel responsible for sending [`Message`]s.
    pub sender: Sender<Message>,
    /// Receiver channel responsible for receiving [`Message`]s.
    pub receiver: Receiver<Message>,
}

impl Connection {
    /// Constructs a new `Connection` with the given sender and receiver
    /// channels.
    pub fn new(sender: Sender<Message>, receiver: Receiver<Message>) -> Self {
        Self { sender, receiver }
    }
}

/// A structure that holds writer and reader threads for sending data to and
/// from the client and server.
pub struct IoThreads {
    /// The writer thread that writes [`Message`]s to standard output.
    writer: thread::JoinHandle<io::Result<()>>,
    /// The reader thread that reads parsed [`Message`]s from standard input.
    reader: thread::JoinHandle<io::Result<()>>,
}

impl IoThreads {
    /// Waits for both writer and reader threads to finish their tasks before
    /// attempting to join them both to the current thread.
    pub fn join(self) -> io::Result<()> {
        match self.writer.join() {
            Ok(result) => result?,
            Err(error) => {
                eprintln!("writer thread failed to join");
                panic!(error)
            }
        };

        match self.reader.join() {
            Ok(result) => result?,
            Err(error) => {
                eprintln!("reader thread failed to join");
                panic!(error)
            }
        };

        Ok(())
    }
}

/// Constructs a new connection over standard I/O.
pub fn stdio() -> (Connection, IoThreads) {
    let (writer_tx, writer_rx) = flume::bounded::<Message>(0);
    let writer = thread::spawn(move || {
        let stdout = std::io::stdout();
        let mut stdout = stdout.lock();

        writer_rx
            .into_iter()
            .try_for_each(|msg| msg.write(&mut stdout))?;

        Ok(())
    });

    let (reader_tx, reader_rx) = flume::bounded::<Message>(0);
    let reader = thread::spawn(move || {
        let stdin = std::io::stdin();
        let mut stdin = stdin.lock();

        while let Some(msg) = Message::read(&mut stdin)? {
            let exit = matches!(&msg, Message::Notification(n) if n.is_exit());
            reader_tx.send(msg).expect("Failed to send to reader");

            if exit {
                break;
            }
        }

        Ok(())
    });

    let connection = Connection::new(writer_tx, reader_rx);
    let threads = IoThreads { writer, reader };
    (connection, threads)
}
