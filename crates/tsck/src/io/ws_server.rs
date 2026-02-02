#![allow(unused)]
pub mod ws {
    use anyhow::Context;
    use flume::{Receiver, Sender, unbounded};
    use std::{
        io::ErrorKind,
        net::{TcpListener, TcpStream},
        thread,
        time::Duration,
    };
    use tungstenite::Message;
    pub type ClientId = u64;

    #[derive(Clone)]
    pub struct Client {
        tx: Sender<Message>,
        id: ClientId,
    }

    impl Client {
        /// Send text without extra allocation when possible
        #[inline]
        pub fn send(&self, text: impl Into<String>) -> bool {
            self.tx.send(Message::Text(text.into().into())).is_ok()
        }

        /// Send text with zero-copy from &str
        #[inline]
        pub fn send_str(&self, text: &str) -> bool {
            self.tx.send(Message::Text(text.to_owned().into())).is_ok()
        }

        #[inline]
        pub fn close(&self) {
            let _ = self.tx.send(Message::Close(None));
        }

        #[inline]
        pub fn id(&self) -> ClientId {
            self.id
        }
    }

    pub enum Event {
        Connected(ClientId, Client),
        Disconnected(ClientId),
        Message(ClientId, String),
    }

    pub struct WsServer {
        rx: Receiver<Event>,
    }

    impl WsServer {
        #[inline]
        pub fn recv(&self) -> anyhow::Result<Event> {
            self.rx.recv().context("server shutdown")
        }

        #[inline]
        pub fn try_recv(&self) -> Option<Event> {
            self.rx.try_recv().ok()
        }

        /// Non-blocking iterator over events
        pub fn events(&self) -> impl Iterator<Item = Event> + '_ {
            std::iter::from_fn(|| self.try_recv())
        }
    }

    /// Launch WebSocket server
    pub fn listen(port: u16) -> anyhow::Result<WsServer> {
        let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).context("bind failed")?;

        let (tx, rx) = unbounded();

        thread::Builder::new()
            .name("ws-accept".into())
            .spawn(move || run_acceptor(listener, tx))
            .context("spawn failed")?;

        Ok(WsServer { rx })
    }

    fn run_acceptor(listener: TcpListener, event_tx: Sender<Event>) {
        let mut id = 0u64;

        for stream in listener.incoming() {
            let Ok(stream) = stream else { continue };

            let current_id = id;
            id = id.wrapping_add(1);

            let tx = event_tx.clone();
            thread::spawn(move || handle_client(stream, current_id, tx));
        }
    }

    fn handle_client(stream: TcpStream, id: ClientId, event_tx: Sender<Event>) {
        const READ_TIMEOUT: Duration = Duration::from_micros(100);

        let _ = stream.set_read_timeout(Some(READ_TIMEOUT));
        let _ = stream.set_nodelay(true);

        let mut ws = match tungstenite::accept(stream) {
            Ok(ws) => ws,
            Err(_) => return,
        };

        let (msg_tx, msg_rx) = unbounded();
        let client = Client { tx: msg_tx, id };

        if event_tx.send(Event::Connected(id, client)).is_err() {
            return;
        }

        loop {
            // Send queued outgoing messages
            while let Ok(msg) = msg_rx.try_recv() {
                if matches!(msg, Message::Close(_)) || ws.send(msg).is_err() {
                    let _ = event_tx.send(Event::Disconnected(id));
                    return;
                }
            }

            // Read incoming messages
            match ws.read() {
                Ok(Message::Text(text)) => {
                    if event_tx.send(Event::Message(id, text.to_string())).is_err() {
                        return;
                    }
                }
                Ok(Message::Close(_)) => break,
                Ok(Message::Ping(data)) => {
                    let _ = ws.send(Message::Pong(data));
                }
                Ok(_) => {} // Ignore other message types
                Err(tungstenite::Error::Io(e))
                    if e.kind() == ErrorKind::WouldBlock || e.kind() == ErrorKind::TimedOut => {}
                Err(tungstenite::Error::ConnectionClosed | tungstenite::Error::AlreadyClosed) => {
                    break;
                }
                Err(_) => break,
            }
        }

        let _ = event_tx.send(Event::Disconnected(id));
    }
}

// enum WsEventExt {
//     AddClient(u64, Client),
//     RemoveClient(u64),
//     OnMessage(u64, String),
// }

// pub struct WsHandler {
//     port: u16,
//     clients: HashMap<u64, Client>,
//     // i need bus. to send to front end
// }
// impl WsHandler {
//     pub fn new(port: u16) -> Self {
//         Self {
//             port,
//             clients: HashMap::new(),
//         }
//     }
//     pub fn send_to(&self, target_id: u64, text: String) {
//         if let Some((_, client)) = self.clients.iter().find(|(id, _)| *id == &target_id) {
//             client.send(&text);
//         }
//     }
//     pub fn send_to_all(&self, text: String) {
//         for (_, client) in self.clients.iter() {
//             client.send(&text);
//         }
//     }

//     pub fn init_ws(&mut self) {
//         let port = self.port.clone();
//         let (tx, rx) = unbounded::<WsEventExt>();
//         let tx = tx.clone();
//         std::thread::spawn(move || -> anyhow::Result<()> {
//             let server = listen(port)?;
//             loop {
//                 match server.recv()? {
//                     Event::Connected(id, client) => {
//                         tx.send(WsEventExt::AddClient(id, client))?;
//                     }

//                     Event::Disconnected(id) => {
//                         println!("Client {} disconnected", id);
//                         tx.send(WsEventExt::RemoveClient(id))?;
//                     }

//                     Event::Message(id, text) => {
//                         println!("Client {}: {}", id, text);
//                         tx.send(WsEventExt::OnMessage(id, text))?;
//                     }
//                 }
//             }
//         });
//     }
// }
