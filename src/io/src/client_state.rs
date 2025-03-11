#[derive(PartialEq, Debug)]
pub enum ConnectionState {
    New,
    Connected,
    Closed,
}