#[cfg(not(feature = "client_host"))]
type Client = ();

#[cfg(feature = "client_host")]
type Client = client::Client;

pub enum ClientConnectionType<'a> {
    Remote,

    /// A Client local to the server's machine. Messages are sent via direct calls. Only used for Singleplayer
    /// and for a player who hosts non-dedicated server.
    Local(&'a Client),
}

pub struct Server {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
