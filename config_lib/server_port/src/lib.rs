pub use server_port_try_from_u16::server_port_try_from_u16;

#[derive(Debug, Clone, serde::Serialize)]
pub struct ServerPort {
    port: std::primitive::u16,
}
impl std::fmt::Display for ServerPort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.port)
    }
}
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ServerPortErrorNamed {
    server_port_min_value: std::primitive::u16,
    server_port_max_value: std::primitive::u16,
    value: std::primitive::u16,
    message: std::string::String,
}
impl std::fmt::Display for ServerPortErrorNamed {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "server_port_min_value: {}, server_port_max_value: {}, value: {}", self.server_port_min_value, self.server_port_max_value, self.value)
    }
}
impl std::convert::TryFrom<std::primitive::u16> for ServerPort {
    type Error = ServerPortErrorNamed;
    fn try_from(value: std::primitive::u16) -> Result<Self, Self::Error> {
        if value < constants::SERVER_PORT_MIN_VALUE {
            Err(Self::Error {
                server_port_min_value: constants::SERVER_PORT_MIN_VALUE,
                server_port_max_value: constants::SERVER_PORT_MAX_VALUE,
                value,
                message: std::string::String::from(constants::SERVER_PORT_IN_SYSTEM_PORT_RANGE_ERROR_MESSAGE),
            })
        } else if value <= constants::SERVER_PORT_MAX_VALUE {
            Ok(Self { port: value })
        } else {
            Err(Self::Error {
                server_port_min_value: constants::SERVER_PORT_MIN_VALUE,
                server_port_max_value: constants::SERVER_PORT_MAX_VALUE,
                value,
                message: std::string::String::from(constants::SERVER_PORT_IN_EPHEMERAL_PORT_RANGE_ERROR_MESSAGE),
            })
        }
    }
}
impl<'de> serde::Deserialize<'de> for ServerPort {
    fn deserialize<D>(deserializer: D) -> Result<ServerPort, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let f = match std::primitive::u16::deserialize(deserializer) {
            Ok(value) => value,
            Err(e) => {
                return Err(e);
            }
        };
        match Self::try_from(f) {
            Ok(value) => Ok(value),
            Err(e) => {
                // return Err("dsfsdf");
                todo!()
            }
        }
    }
}