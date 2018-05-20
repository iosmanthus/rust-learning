pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        super::network::client::display();
    }
}
