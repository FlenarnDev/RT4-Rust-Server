#[derive(PartialEq, Debug)]
pub enum ConnectionState {
    New,
    Connected,
    Login,
    Reconnect,
    Logout,
    Closed,
}