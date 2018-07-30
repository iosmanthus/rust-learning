pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::client::display();
        super::network::client::display();
    }
}
