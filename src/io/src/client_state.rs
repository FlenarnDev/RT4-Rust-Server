#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ClientState {
    Closed,
    New,
    Js5,
    Login,
    Login_Secondary,
    Other,
}