pub trait GetServerAddress {
    fn get_server_address(&self) -> std::string::String;
}

impl<SelfGeneric> GetServerAddress for SelfGeneric
where
    Self: app_state::GetServerHost + app_state::GetServerPort,
{
    fn get_server_address(&self) -> std::string::String {
        format!("{}:{}", *self.get_server_host(), *self.get_server_port())
    }
}
