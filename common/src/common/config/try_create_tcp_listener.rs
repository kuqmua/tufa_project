pub trait TryCreateTcpListener<'a> {
    fn try_create_tcp_listener(&self) -> Result<std::net::TcpListener, Box<crate::common::config::try_create_tcp_listener::TryCreateTcpListenerErrorNamed>>;
}

#[derive(Debug, thiserror::Error, error_occurence_lib::ErrorOccurence)] //
pub enum TryCreateTcpListenerErrorNamed {
    TcpListenerBind {
        #[eo_to_std_string_string]
        tcp_listener_bind: std::io::Error,
        code_occurence: error_occurence_lib::code_occurence::CodeOccurence,
    },
}

impl<SelfGeneric> TryCreateTcpListener<'_> for SelfGeneric
where
    Self: app_state::GetServiceSocketAddress,
{
    fn try_create_tcp_listener(&self) -> Result<std::net::TcpListener, Box<crate::common::config::try_create_tcp_listener::TryCreateTcpListenerErrorNamed>> {
        match std::net::TcpListener::bind(self.get_service_socket_address()) {
            Ok(listener) => Ok(listener),
            Err(error) => Err(Box::new(crate::common::config::try_create_tcp_listener::TryCreateTcpListenerErrorNamed::TcpListenerBind {
                tcp_listener_bind: error,
                code_occurence: error_occurence_lib::code_occurence!(),
            })),
        }
    }
}
