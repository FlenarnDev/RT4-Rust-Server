use tokio::net::TcpStream;

#[derive(Debug)]
pub enum ClientState {
    CLOSED,
    CONNECTED,
    JS5,
    WORLDLIST,
    LOGIN,
    GAME,
    PROXYING(TcpStream),
}

impl PartialEq for ClientState {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CLOSED, Self::CLOSED) => true,
            (Self::CONNECTED, Self::CONNECTED) => true,
            (Self::JS5, Self::JS5) => true,
            (Self::WORLDLIST, Self::WORLDLIST) => true,
            (Self::LOGIN, Self::LOGIN) => true,
            (Self::GAME, Self::GAME) => true,
            (Self::PROXYING(_), Self::PROXYING(_)) => true,
            _ => false,
        }
    }
}