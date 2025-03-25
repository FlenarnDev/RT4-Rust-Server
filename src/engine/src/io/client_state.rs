#[derive(Clone, PartialEq, Debug)]
pub enum ConnectionState {
    New,
    Connected,
    Login,
    Reconnect,
    Logout,
    Closed,
    Null,
}